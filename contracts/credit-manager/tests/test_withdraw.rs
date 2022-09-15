use cosmwasm_std::OverflowOperation::Sub;
use cosmwasm_std::{Addr, Coin, OverflowError, Uint128};

use rover::error::ContractError;
use rover::error::ContractError::{NotTokenOwner, NotWhitelisted};
use rover::msg::execute::Action;

use crate::helpers::{assert_err, uatom_info, ujake_info, uosmo_info, AccountToFund, MockEnv};

pub mod helpers;

#[test]
fn test_only_owner_of_token_can_withdraw() {
    let coin_info = uosmo_info();
    let owner = Addr::unchecked("owner");
    let mut mock = MockEnv::new().build().unwrap();
    let token_id = mock.create_credit_account(&owner).unwrap();

    let another_user = Addr::unchecked("another_user");
    let res = mock.update_credit_account(
        &token_id,
        &another_user,
        vec![Action::Withdraw(Coin {
            denom: coin_info.denom,
            amount: Uint128::new(382),
        })],
        &[],
    );

    assert_err(
        res,
        NotTokenOwner {
            user: another_user.into(),
            token_id: token_id.clone(),
        },
    );

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 0);
}

#[test]
fn test_withdraw_nothing() {
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
        vec![Action::Withdraw(Coin {
            denom: coin_info.denom,
            amount: Uint128::new(0),
        })],
        &[],
    );

    assert_err(res, ContractError::NoAmount);

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 0);
}

#[test]
fn test_withdraw_but_no_funds() {
    let coin_info = uosmo_info();
    let user = Addr::unchecked("user");
    let mut mock = MockEnv::new()
        .allowed_coins(&[coin_info.clone()])
        .build()
        .unwrap();
    let token_id = mock.create_credit_account(&user).unwrap();

    let withdraw_amount = Uint128::new(234);
    let res = mock.update_credit_account(
        &token_id,
        &user,
        vec![Action::Withdraw(coin_info.to_coin(withdraw_amount))],
        &[],
    );

    assert_err(
        res,
        ContractError::Overflow(OverflowError {
            operation: Sub,
            operand1: "0".to_string(),
            operand2: "234".to_string(),
        }),
    );

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 0);
}

#[test]
fn test_withdraw_but_not_enough_funds() {
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

    let res = mock.update_credit_account(
        &token_id,
        &user,
        vec![
            Action::Deposit(coin_info.to_coin(Uint128::new(300))),
            Action::Withdraw(coin_info.to_coin(Uint128::new(400))),
        ],
        &[Coin::new(300u128, coin_info.denom)],
    );

    assert_err(
        res,
        ContractError::Overflow(OverflowError {
            operation: Sub,
            operand1: "300".to_string(),
            operand2: "400".to_string(),
        }),
    );

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 0);
}

#[test]
fn test_can_only_withdraw_allowed_assets() {
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

    let not_allowed_coin = ujake_info().to_coin(Uint128::new(234));
    let res = mock.update_credit_account(
        &token_id,
        &user,
        vec![
            Action::Deposit(coin_info.to_coin(Uint128::new(234))),
            Action::Withdraw(not_allowed_coin.clone()),
        ],
        &[Coin::new(234u128, coin_info.denom)],
    );

    assert_err(res, NotWhitelisted(not_allowed_coin.denom));

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 0);
}

#[test]
fn test_cannot_withdraw_more_than_healthy() {
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

    let deposit_amount = Uint128::new(200);
    let res = mock.update_credit_account(
        &token_id,
        &user,
        vec![
            Action::Deposit(coin_info.to_coin(deposit_amount)),
            Action::Borrow(coin_info.to_coin(Uint128::new(400))),
            Action::Withdraw(coin_info.to_coin(Uint128::new(50))),
        ],
        &[Coin::new(200u128, coin_info.denom)],
    );

    assert_err(
        res,
        ContractError::AboveMaxLTV {
            token_id: token_id.clone(),
            max_ltv_health_factor: "0.960099750623441396".to_string(),
        },
    );

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 0);
}

#[test]
fn test_withdraw_success() {
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

    let deposit_amount = Uint128::new(234);
    mock.update_credit_account(
        &token_id,
        &user,
        vec![
            Action::Deposit(coin_info.to_coin(deposit_amount)),
            Action::Withdraw(coin_info.to_coin(deposit_amount)),
        ],
        &[Coin::new(deposit_amount.into(), coin_info.denom.clone())],
    )
    .unwrap();

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 0);

    let coin = mock.query_balance(&mock.rover, &coin_info.denom);
    assert_eq!(coin.amount, Uint128::zero())
}

#[test]
fn test_multiple_withdraw_actions() {
    let uosmo_info = uosmo_info();
    let uatom_info = uatom_info();

    let user = Addr::unchecked("user");
    let mut mock = MockEnv::new()
        .allowed_coins(&[uosmo_info.clone(), uatom_info.clone()])
        .fund_account(AccountToFund {
            addr: user.clone(),
            funds: vec![
                Coin::new(234u128, uosmo_info.denom.clone()),
                Coin::new(25u128, uatom_info.denom.clone()),
            ],
        })
        .build()
        .unwrap();
    let token_id = mock.create_credit_account(&user).unwrap();

    let uosmo_amount = Uint128::new(234);
    let uatom_amount = Uint128::new(25);

    mock.update_credit_account(
        &token_id,
        &user,
        vec![
            Action::Deposit(uosmo_info.to_coin(uosmo_amount)),
            Action::Deposit(uatom_info.to_coin(uatom_amount)),
        ],
        &[
            Coin::new(234u128, uosmo_info.denom.clone()),
            Coin::new(25u128, uatom_info.denom.clone()),
        ],
    )
    .unwrap();

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 2);

    let coin = mock.query_balance(&user, &uosmo_info.denom);
    assert_eq!(coin.amount, Uint128::zero());

    let coin = mock.query_balance(&user, &uatom_info.denom);
    assert_eq!(coin.amount, Uint128::zero());

    mock.update_credit_account(
        &token_id,
        &user,
        vec![Action::Withdraw(uosmo_info.to_coin(uosmo_amount))],
        &[],
    )
    .unwrap();

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 1);

    let coin = mock.query_balance(&mock.rover, &uosmo_info.denom);
    assert_eq!(coin.amount, Uint128::zero());

    let coin = mock.query_balance(&user, &uosmo_info.denom);
    assert_eq!(coin.amount, uosmo_amount);

    mock.update_credit_account(
        &token_id,
        &user,
        vec![Action::Withdraw(uatom_info.to_coin(Uint128::new(20)))],
        &[],
    )
    .unwrap();

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 1);

    let coin = mock.query_balance(&mock.rover, &uatom_info.denom);
    assert_eq!(coin.amount, Uint128::new(5));

    let coin = mock.query_balance(&user, &uatom_info.denom);
    assert_eq!(coin.amount, Uint128::new(20));

    mock.update_credit_account(
        &token_id,
        &user,
        vec![Action::Withdraw(uatom_info.to_coin(Uint128::new(5)))],
        &[],
    )
    .unwrap();

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 0);

    let coin = mock.query_balance(&mock.rover, &uatom_info.denom);
    assert_eq!(coin.amount, Uint128::zero());

    let coin = mock.query_balance(&user, &uatom_info.denom);
    assert_eq!(coin.amount, uatom_amount);
}