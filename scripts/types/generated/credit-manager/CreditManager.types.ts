// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.16.4.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

export type VaultBaseForString = string
export type Decimal = string
export type OracleBaseForString = string
export type RedBankBaseForString = string
export type SwapperBaseForString = string
export interface InstantiateMsg {
  allowed_coins: string[]
  allowed_vaults: VaultBaseForString[]
  max_close_factor: Decimal
  max_liquidation_bonus: Decimal
  oracle: OracleBaseForString
  owner: string
  red_bank: RedBankBaseForString
  swapper: SwapperBaseForString
  [k: string]: unknown
}
export type ExecuteMsg =
  | {
      create_credit_account: {
        [k: string]: unknown
      }
    }
  | {
      update_credit_account: {
        account_id: string
        actions: Action[]
        [k: string]: unknown
      }
    }
  | {
      update_config: {
        new_config: ConfigUpdates
        [k: string]: unknown
      }
    }
  | {
      callback: CallbackMsg
    }
export type Action =
  | {
      deposit: Coin
    }
  | {
      withdraw: Coin
    }
  | {
      borrow: Coin
    }
  | {
      repay: Coin
    }
  | {
      vault_deposit: {
        coins: Coin[]
        vault: VaultBaseForString
        [k: string]: unknown
      }
    }
  | {
      vault_withdraw: {
        amount: Uint128
        vault: VaultBaseForString
        [k: string]: unknown
      }
    }
  | {
      liquidate_coin: {
        debt_coin: Coin
        liquidatee_account_id: string
        request_coin_denom: string
        [k: string]: unknown
      }
    }
  | {
      swap_exact_in: {
        coin_in: Coin
        denom_out: string
        slippage: Decimal
        [k: string]: unknown
      }
    }
export type Uint128 = string
export type CallbackMsg =
  | {
      withdraw: {
        account_id: string
        coin: Coin
        recipient: Addr
        [k: string]: unknown
      }
    }
  | {
      borrow: {
        account_id: string
        coin: Coin
        [k: string]: unknown
      }
    }
  | {
      repay: {
        account_id: string
        coin: Coin
        [k: string]: unknown
      }
    }
  | {
      assert_below_max_l_t_v: {
        account_id: string
        [k: string]: unknown
      }
    }
  | {
      vault_deposit: {
        account_id: string
        coins: Coin[]
        vault: VaultBaseForAddr
        [k: string]: unknown
      }
    }
  | {
      update_vault_coin_balance: {
        account_id: string
        previous_total_balance: Uint128
        vault: VaultBaseForAddr1
        [k: string]: unknown
      }
    }
  | {
      vault_withdraw: {
        account_id: string
        amount: Uint128
        vault: VaultBaseForAddr2
        [k: string]: unknown
      }
    }
  | {
      vault_force_withdraw: {
        account_id: string
        amount: Uint128
        vault: VaultBaseForAddr3
        [k: string]: unknown
      }
    }
  | {
      liquidate_coin: {
        debt_coin: Coin
        liquidatee_account_id: string
        liquidator_account_id: string
        request_coin_denom: string
        [k: string]: unknown
      }
    }
  | {
      assert_health_factor_improved: {
        account_id: string
        previous_health_factor: Decimal
        [k: string]: unknown
      }
    }
  | {
      swap_exact_in: {
        account_id: string
        coin_in: Coin
        denom_out: string
        slippage: Decimal
        [k: string]: unknown
      }
    }
  | {
      update_coin_balances: {
        account_id: string
        previous_balances: Coin[]
        [k: string]: unknown
      }
    }
export type Addr = string
export type VaultBaseForAddr = string
export type VaultBaseForAddr1 = string
export type VaultBaseForAddr2 = string
export type VaultBaseForAddr3 = string
export interface Coin {
  amount: Uint128
  denom: string
  [k: string]: unknown
}
export interface ConfigUpdates {
  account_nft?: string | null
  allowed_coins?: string[] | null
  allowed_vaults?: VaultBaseForString[] | null
  max_close_factor?: Decimal | null
  max_liquidation_bonus?: Decimal | null
  oracle?: OracleBaseForString | null
  owner?: string | null
  red_bank?: RedBankBaseForString | null
  swapper?: SwapperBaseForString | null
  [k: string]: unknown
}
export type QueryMsg =
  | {
      config: {}
    }
  | {
      allowed_vaults: {
        limit?: number | null
        start_after?: VaultBaseForString | null
      }
    }
  | {
      allowed_coins: {
        limit?: number | null
        start_after?: string | null
      }
    }
  | {
      positions: {
        account_id: string
      }
    }
  | {
      health: {
        account_id: string
      }
    }
  | {
      all_coin_balances: {
        limit?: number | null
        start_after?: [string, string] | null
      }
    }
  | {
      all_debt_shares: {
        limit?: number | null
        start_after?: [string, string] | null
      }
    }
  | {
      total_debt_shares: string
    }
  | {
      all_total_debt_shares: {
        limit?: number | null
        start_after?: string | null
      }
    }
  | {
      all_vault_positions: {
        limit?: number | null
        start_after?: [string, string] | null
      }
    }
  | {
      total_vault_coin_balance: {
        vault: VaultBaseForString
      }
    }
  | {
      all_total_vault_coin_balances: {
        limit?: number | null
        start_after?: VaultBaseForString | null
      }
    }
export type ArrayOfCoinBalanceResponseItem = CoinBalanceResponseItem[]
export interface CoinBalanceResponseItem {
  account_id: string
  amount: Uint128
  denom: string
  [k: string]: unknown
}
export type ArrayOfSharesResponseItem = SharesResponseItem[]
export interface SharesResponseItem {
  account_id: string
  denom: string
  shares: Uint128
  [k: string]: unknown
}
export type ArrayOfDebtShares = DebtShares[]
export interface DebtShares {
  denom: string
  shares: Uint128
  [k: string]: unknown
}
export type ArrayOfVaultWithBalance = VaultWithBalance[]
export interface VaultWithBalance {
  balance: Uint128
  vault: VaultBaseForString
  [k: string]: unknown
}
export type ArrayOfVaultPositionResponseItem = VaultPositionResponseItem[]
export interface VaultPositionResponseItem {
  account_id: string
  addr: string
  vault_position: VaultPosition
  [k: string]: unknown
}
export interface VaultPosition {
  locked: Uint128
  unlocked: Uint128
  [k: string]: unknown
}
export type ArrayOfString = string[]
export type ArrayOfVaultBaseForString = VaultBaseForString[]
export interface ConfigResponse {
  account_nft?: string | null
  max_close_factor: Decimal
  max_liquidation_bonus: Decimal
  oracle: string
  owner: string
  red_bank: string
  swapper: string
  [k: string]: unknown
}
export interface HealthResponse {
  above_max_ltv: boolean
  liquidatable: boolean
  liquidation_health_factor?: Decimal | null
  liquidation_threshold_adjusted_collateral: Decimal
  max_ltv_adjusted_collateral: Decimal
  max_ltv_health_factor?: Decimal | null
  total_collateral_value: Decimal
  total_debt_value: Decimal
  [k: string]: unknown
}
export interface PositionsWithValueResponse {
  account_id: string
  coins: CoinValue[]
  debt: DebtSharesValue[]
  vault_positions: VaultPositionWithAddr[]
  [k: string]: unknown
}
export interface CoinValue {
  amount: Uint128
  denom: string
  price: Decimal
  value: Decimal
  [k: string]: unknown
}
export interface DebtSharesValue {
  amount: Uint128
  denom: string
  price: Decimal
  shares: Uint128
  value: Decimal
  [k: string]: unknown
}
export interface VaultPositionWithAddr {
  addr: string
  position: VaultPosition
  [k: string]: unknown
}
