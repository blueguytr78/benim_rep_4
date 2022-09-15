use std::ops::{Mul, Sub};

use cosmwasm_std::{Addr, Coin, Decimal, Uint128};

use credit_manager::borrow::DEFAULT_DEBT_SHARES_PER_COIN_BORROWED;
use rover::error::ContractError;
use rover::msg::execute::Action::{Borrow, Deposit};

use crate::helpers::{
    assert_err, uosmo_info, AccountToFund, MockEnv, DEFAULT_RED_BANK_COIN_BALANCE,
};

pub mod helpers;

#[test]
fn test_only_token_owner_can_borrow() {
    let coin_info = uosmo_info();

    let user = Addr::unchecked("user");
    let mut mock = MockEnv::new()
        .allowed_coins(&[coin_info.clone()])
        .build()
        .unwrap();
    let token_id = mock.create_credit_account(&user).unwrap();

    let another_user = Addr::unchecked("another_user");
    let res = mock.update_credit_account(
        &token_id,
        &another_user,
        vec![Borrow(coin_info.to_coin(Uint128::new(12312u128)))],
        &[],
    );

    assert_err(
        res,
        ContractError::NotTokenOwner {
            user: another_user.into(),
            token_id,
        },
    )
}

#[test]
fn test_can_only_borrow_what_is_whitelisted() {
    let coin_info = uosmo_info();

    let user = Addr::unchecked("user");
    let mut mock = MockEnv::new().allowed_coins(&[coin_info]).build().unwrap();
    let token_id = mock.create_credit_account(&user).unwrap();

    let res = mock.update_credit_account(
        &token_id,
        &user,
        vec![Borrow(Coin {
            denom: "usomething".to_string(),
            amount: Uint128::new(234),
        })],
        &[],
    );

    assert_err(
        res,
        ContractError::NotWhitelisted(String::from("usomething")),
    )
}

#[test]
fn test_borrowing_zero_does_nothing() {
    let coin_info = uosmo_info();

    let user = Addr::unchecked("user");
    let mut mock = MockEnv::new()
        .allowed_coins(&[coin_info.clone()])
        .build()
        .unwrap();
    let token_id = mock.create_credit_account(&user).unwrap();

    let res = mock.update_credit_account(
        &token_id,
        &user,
        vec![Borrow(coin_info.to_coin(Uint128::zero()))],
        &[],
    );

    assert_err(res, ContractError::NoAmount);

    let position = mock.query_position(&token_id);
    assert_eq!(position.coins.len(), 0);
    assert_eq!(position.debt.len(), 0);
}

#[test]
fn test_cannot_borrow_above_max_ltv() {
    let coin_info = uosmo_info();
    let user = Addr::unchecked("user");
    let mut mock = MockEnv::new()
        .allowed_coins(&[coin_info.clone()])
        .fund_account(AccountToFund {
            addr: user.clone(),
            funds: vec![Coin::new(300u128, coin_info.denom.clone())],
        })
        .build()
        .unwrap();
    let token_id = mock.create_credit_account(&user).unwrap();

    let position = mock.query_position(&token_id);
    assert_eq!(position.coins.len(), 0);
    assert_eq!(position.debt.len(), 0);

    let res = mock.update_credit_account(
        &token_id,
        &user,
        vec![
            Deposit(coin_info.to_coin(Uint128::new(300))),
            Borrow(coin_info.to_coin(Uint128::new(700))),
        ],
        &[Coin::new(300u128, coin_info.denom)],
    );

    assert_err(
        res,
        ContractError::AboveMaxLTV {
            token_id,
            max_ltv_health_factor: "0.998573466476462196".to_string(),
        },
    );
}

#[test]
fn test_success_when_new_debt_asset() {
    let coin_info = uosmo_info();
    let user = Addr::unchecked("user");
    let mut mock = MockEnv::new()
        .allowed_coins(&[coin_info.clone()])
        .fund_account(AccountToFund {
            addr: user.clone(),
            funds: vec![Coin::new(300u128, coin_info.denom.clone())],
        })
        .build()
        .unwrap();
    let token_id = mock.create_credit_account(&user).unwrap();

    let position = mock.query_position(&token_id);
    assert_eq!(position.coins.len(), 0);
    assert_eq!(position.debt.len(), 0);
    mock.update_credit_account(
        &token_id,
        &user,
        vec![
            Deposit(Coin {
                denom: coin_info.denom.clone(),
                amount: Uint128::new(300),
            }),
            Borrow(Coin {
                denom: coin_info.denom.clone(),
                amount: Uint128::new(42),
            }),
        ],
        &[Coin::new(300u128, coin_info.denom.clone())],
    )
    .unwrap();

    let position = mock.query_position(&token_id);
    assert_eq!(position.coins.len(), 1);
    let asset_res = position.coins.first().unwrap();
    assert_eq!(
        asset_res.amount,
        Uint128::new(342) // Deposit + Borrow
    );
    assert_eq!(asset_res.denom, coin_info.denom);
    assert_eq!(asset_res.price, coin_info.price);
    assert_eq!(
        asset_res.value,
        coin_info.price * Decimal::from_atomics(342u128, 0).unwrap()
    );

    let debt_shares_res = position.debt.first().unwrap();
    assert_eq!(position.debt.len(), 1);
    assert_eq!(
        debt_shares_res.shares,
        Uint128::new(42).mul(DEFAULT_DEBT_SHARES_PER_COIN_BORROWED)
    );
    assert_eq!(debt_shares_res.denom, coin_info.denom);
    let debt_amount = Uint128::new(42u128) + Uint128::new(1); // simulated yield
    assert_eq!(
        debt_shares_res.value,
        coin_info.price * Decimal::from_atomics(debt_amount, 0).unwrap()
    );

    let coin = mock.query_balance(&mock.rover, &coin_info.denom);
    assert_eq!(coin.amount, Uint128::new(342));

    let config = mock.query_config();
    let coin = mock.query_balance(&Addr::unchecked(config.red_bank), &coin_info.denom);
    assert_eq!(
        coin.amount,
        DEFAULT_RED_BANK_COIN_BALANCE.sub(Uint128::new(42))
    );

    let res = mock.query_total_debt_shares(&coin_info.denom);
    assert_eq!(
        res.shares,
        Uint128::new(42).mul(DEFAULT_DEBT_SHARES_PER_COIN_BORROWED)
    );
}

#[test]
fn test_debt_shares_with_debt_amount() {
    let coin_info = uosmo_info();
    let user_a = Addr::unchecked("user_a");
    let user_b = Addr::unchecked("user_b");
    let mut mock = MockEnv::new()
        .allowed_coins(&[coin_info.clone()])
        .fund_account(AccountToFund {
            addr: user_a.clone(),
            funds: vec![Coin::new(300u128, coin_info.denom.clone())],
        })
        .fund_account(AccountToFund {
            addr: user_b.clone(),
            funds: vec![Coin::new(450u128, coin_info.denom.clone())],
        })
        .build()
        .unwrap();
    let token_id_a = mock.create_credit_account(&user_a).unwrap();
    let token_id_b = mock.create_credit_account(&user_b).unwrap();

    mock.update_credit_account(
        &token_id_a,
        &user_a,
        vec![
            Deposit(coin_info.to_coin(Uint128::new(300))),
            Borrow(coin_info.to_coin(Uint128::new(50))),
        ],
        &[Coin::new(300u128, coin_info.denom.clone())],
    )
    .unwrap();

    let interim_red_bank_debt = mock.query_red_bank_debt(&coin_info.denom);

    mock.update_credit_account(
        &token_id_b,
        &user_b,
        vec![
            Deposit(coin_info.to_coin(Uint128::new(450))),
            Borrow(coin_info.to_coin(Uint128::new(50))),
        ],
        &[Coin::new(450u128, coin_info.denom.clone())],
    )
    .unwrap();

    let token_a_shares = Uint128::new(50).mul(DEFAULT_DEBT_SHARES_PER_COIN_BORROWED);
    let position = mock.query_position(&token_id_a);
    let debt_position_a = position.debt.first().unwrap();
    assert_eq!(debt_position_a.shares, token_a_shares.clone());
    assert_eq!(debt_position_a.denom, coin_info.denom);

    let token_b_shares = Uint128::new(50)
        .mul(DEFAULT_DEBT_SHARES_PER_COIN_BORROWED)
        .multiply_ratio(Uint128::new(50), interim_red_bank_debt.amount);
    let position = mock.query_position(&token_id_b);
    let debt_position_b = position.debt.first().unwrap();
    assert_eq!(debt_position_b.shares, token_b_shares.clone());
    assert_eq!(debt_position_b.denom, coin_info.denom);

    let total = mock.query_total_debt_shares(&coin_info.denom);

    assert_eq!(
        total.shares,
        debt_position_a.shares + debt_position_b.shares
    );

    let red_bank_debt = mock.query_red_bank_debt(&coin_info.denom);

    let a_amount_owed = red_bank_debt
        .amount
        .multiply_ratio(debt_position_a.shares, total.shares);
    assert_eq!(
        debt_position_a.value,
        coin_info.price * Decimal::from_atomics(a_amount_owed, 0).unwrap()
    );

    let b_amount_owed = red_bank_debt
        .amount
        .multiply_ratio(debt_position_b.shares, total.shares);
    assert_eq!(
        debt_position_b.value,
        coin_info.price * Decimal::from_atomics(b_amount_owed, 0).unwrap()
    );

    // NOTE: There is an expected rounding error. This will not pass.
    // let total_borrowed_plus_interest = Decimal::from_atomics(Uint128::new(102), 0).unwrap();
    // assert_eq!(
    //     total_borrowed_plus_interest * coin_info.price,
    //     debt_position_a.total_value + debt_position_b.total_value
    // )
    // This test below asserts the rounding down that's happening
    let total_owed = Decimal::from_atomics(a_amount_owed + b_amount_owed, 0).unwrap();
    assert_eq!(
        total_owed * coin_info.price,
        debt_position_a.value + debt_position_b.value
    )
}