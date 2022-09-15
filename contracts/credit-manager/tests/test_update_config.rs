use cosmwasm_std::{Addr, Decimal};

use rover::adapters::{OracleBase, RedBankBase, VaultBase};
use rover::msg::instantiate::ConfigUpdates;

use crate::helpers::MockEnv;

pub mod helpers;

#[test]
fn test_only_owner_can_update_config() {
    let mut mock = MockEnv::new().build().unwrap();
    let new_owner = Addr::unchecked("bad_guy");

    let res = mock.update_config(
        &new_owner,
        ConfigUpdates {
            account_nft: None,
            owner: Some(new_owner.to_string()),
            allowed_vaults: None,
            allowed_coins: None,
            red_bank: None,
            oracle: None,
            max_liquidation_bonus: None,
            max_close_factor: None,
        },
    );

    if res.is_ok() {
        panic!("only owner should be able to update config");
    }
}

#[test]
fn test_update_config_works_with_full_config() {
    let mut mock = MockEnv::new().build().unwrap();
    let original_config = mock.query_config();
    let original_allowed_vaults = mock.query_allowed_vaults(None, None);
    let original_allowed_coins = mock.query_allowed_coins(None, None);

    let new_nft_contract = mock.deploy_nft_contract().unwrap();
    let new_owner = Addr::unchecked("new_owner");
    let new_red_bank = RedBankBase::new("new_red_bank".to_string());
    let new_allowed_vaults = vec![VaultBase::new("vault_contract_1".to_string())];
    let new_allowed_coins = vec!["uosmo".to_string()];
    let new_oracle = OracleBase::new("new_oracle".to_string());
    let new_liq_bonus = Decimal::from_atomics(17u128, 2).unwrap();
    let new_close_factor = Decimal::from_atomics(32u128, 2).unwrap();

    mock.update_config(
        &Addr::unchecked(original_config.owner.clone()),
        ConfigUpdates {
            account_nft: Some(new_nft_contract.to_string()),
            owner: Some(new_owner.to_string()),
            allowed_vaults: Some(new_allowed_vaults.clone()),
            allowed_coins: Some(new_allowed_coins.clone()),
            red_bank: Some(new_red_bank.clone()),
            oracle: Some(new_oracle.clone()),
            max_liquidation_bonus: Some(new_liq_bonus),
            max_close_factor: Some(new_close_factor),
        },
    )
    .unwrap();

    let new_config = mock.query_config();
    let new_queried_allowed_vaults = mock.query_allowed_vaults(None, None);
    let new_queried_allowed_coins = mock.query_allowed_coins(None, None);

    assert_eq!(new_config.account_nft, Some(new_nft_contract.to_string()));
    assert_ne!(new_config.account_nft, original_config.account_nft);

    assert_eq!(new_config.owner, new_owner.to_string());
    assert_ne!(new_config.owner, original_config.owner);

    assert_eq!(new_queried_allowed_vaults, new_allowed_vaults);
    assert_ne!(new_queried_allowed_vaults, original_allowed_vaults);

    assert_eq!(new_queried_allowed_coins, new_allowed_coins);
    assert_ne!(new_queried_allowed_coins, original_allowed_coins);

    assert_eq!(&new_config.red_bank, new_red_bank.address());
    assert_ne!(new_config.red_bank, original_config.red_bank);

    assert_eq!(&new_config.oracle, new_oracle.address());
    assert_ne!(new_config.oracle, original_config.oracle);

    assert_eq!(new_config.max_liquidation_bonus, new_liq_bonus);
    assert_ne!(
        new_config.max_liquidation_bonus,
        original_config.max_liquidation_bonus
    );

    assert_eq!(new_config.max_close_factor, new_close_factor);
    assert_ne!(
        new_config.max_close_factor,
        original_config.max_close_factor
    );
}

#[test]
fn test_update_config_works_with_some_config() {
    let mut mock = MockEnv::new().build().unwrap();
    let original_config = mock.query_config();
    let original_allowed_vaults = mock.query_allowed_vaults(None, None);
    let original_allowed_coins = mock.query_allowed_coins(None, None);

    let new_nft_contract = mock.deploy_nft_contract().unwrap();
    let new_allowed_vaults = vec![VaultBase::new("vault_contract_1".to_string())];

    mock.update_config(
        &Addr::unchecked(original_config.owner.clone()),
        ConfigUpdates {
            account_nft: Some(new_nft_contract.to_string()),
            allowed_vaults: Some(new_allowed_vaults.clone()),
            ..Default::default()
        },
    )
    .unwrap();

    let new_config = mock.query_config();
    let new_queried_allowed_vaults = mock.query_allowed_vaults(None, None);
    let new_queried_allowed_coins = mock.query_allowed_coins(None, None);

    // Changed configs
    assert_eq!(new_config.account_nft, Some(new_nft_contract.to_string()));
    assert_ne!(new_config.account_nft, original_config.account_nft);

    assert_eq!(new_queried_allowed_vaults, new_allowed_vaults);
    assert_ne!(new_queried_allowed_vaults, original_allowed_vaults);

    // Unchanged configs
    assert_eq!(new_config.owner, original_config.owner);
    assert_eq!(original_allowed_coins, new_queried_allowed_coins);
    assert_eq!(new_config.red_bank, original_config.red_bank);
}

#[test]
fn test_update_config_does_nothing_when_nothing_is_passed() {
    let mut mock = MockEnv::new().build().unwrap();
    let original_config = mock.query_config();
    let original_allowed_vaults = mock.query_allowed_vaults(None, None);
    let original_allowed_coins = mock.query_allowed_coins(None, None);

    mock.update_config(
        &Addr::unchecked(original_config.owner.clone()),
        Default::default(),
    )
    .unwrap();

    let new_config = mock.query_config();
    let new_queried_allowed_vaults = mock.query_allowed_vaults(None, None);
    let new_queried_allowed_coins = mock.query_allowed_coins(None, None);

    assert_eq!(new_config.account_nft, original_config.account_nft);
    assert_eq!(new_config.owner, original_config.owner);
    assert_eq!(new_queried_allowed_vaults, original_allowed_vaults);
    assert_eq!(new_queried_allowed_coins, original_allowed_coins);
    assert_eq!(new_config.red_bank, original_config.red_bank);
    assert_eq!(new_config.oracle, original_config.oracle);
    assert_eq!(
        new_config.max_liquidation_bonus,
        original_config.max_liquidation_bonus
    );
    assert_eq!(
        new_config.max_close_factor,
        original_config.max_close_factor
    );
}
