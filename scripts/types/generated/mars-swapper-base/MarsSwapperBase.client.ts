// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.24.0.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from '@cosmjs/cosmwasm-stargate'
import { StdFee } from '@cosmjs/amino'
import {
  InstantiateMsg,
  ExecuteMsg,
  AdminUpdate,
  Uint128,
  Decimal,
  Addr,
  Empty,
  Coin,
  QueryMsg,
  AdminResponse,
  EstimateExactInSwapResponse,
  RouteResponseForEmpty,
  ArrayOfRouteResponseForEmpty,
} from './MarsSwapperBase.types'
export interface MarsSwapperBaseReadOnlyInterface {
  contractAddress: string
  admin: () => Promise<AdminResponse>
  route: ({
    denomIn,
    denomOut,
  }: {
    denomIn: string
    denomOut: string
  }) => Promise<RouteResponseForEmpty>
  routes: ({
    limit,
    startAfter,
  }: {
    limit?: number
    startAfter?: string[][]
  }) => Promise<ArrayOfRouteResponseForEmpty>
  estimateExactInSwap: ({
    coinIn,
    denomOut,
  }: {
    coinIn: Coin
    denomOut: string
  }) => Promise<EstimateExactInSwapResponse>
}
export class MarsSwapperBaseQueryClient implements MarsSwapperBaseReadOnlyInterface {
  client: CosmWasmClient
  contractAddress: string

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client
    this.contractAddress = contractAddress
    this.admin = this.admin.bind(this)
    this.route = this.route.bind(this)
    this.routes = this.routes.bind(this)
    this.estimateExactInSwap = this.estimateExactInSwap.bind(this)
  }

  admin = async (): Promise<AdminResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      admin: {},
    })
  }
  route = async ({
    denomIn,
    denomOut,
  }: {
    denomIn: string
    denomOut: string
  }): Promise<RouteResponseForEmpty> => {
    return this.client.queryContractSmart(this.contractAddress, {
      route: {
        denom_in: denomIn,
        denom_out: denomOut,
      },
    })
  }
  routes = async ({
    limit,
    startAfter,
  }: {
    limit?: number
    startAfter?: string[][]
  }): Promise<ArrayOfRouteResponseForEmpty> => {
    return this.client.queryContractSmart(this.contractAddress, {
      routes: {
        limit,
        start_after: startAfter,
      },
    })
  }
  estimateExactInSwap = async ({
    coinIn,
    denomOut,
  }: {
    coinIn: Coin
    denomOut: string
  }): Promise<EstimateExactInSwapResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      estimate_exact_in_swap: {
        coin_in: coinIn,
        denom_out: denomOut,
      },
    })
  }
}
export interface MarsSwapperBaseInterface extends MarsSwapperBaseReadOnlyInterface {
  contractAddress: string
  sender: string
  updateAdmin: (
    fee?: number | StdFee | 'auto',
    memo?: string,
    funds?: Coin[],
  ) => Promise<ExecuteResult>
  setRoute: (
    {
      denomIn,
      denomOut,
      route,
    }: {
      denomIn: string
      denomOut: string
      route: Empty
    },
    fee?: number | StdFee | 'auto',
    memo?: string,
    funds?: Coin[],
  ) => Promise<ExecuteResult>
  swapExactIn: (
    {
      coinIn,
      denomOut,
      slippage,
    }: {
      coinIn: Coin
      denomOut: string
      slippage: Decimal
    },
    fee?: number | StdFee | 'auto',
    memo?: string,
    funds?: Coin[],
  ) => Promise<ExecuteResult>
  transferResult: (
    {
      denomIn,
      denomOut,
      recipient,
    }: {
      denomIn: string
      denomOut: string
      recipient: Addr
    },
    fee?: number | StdFee | 'auto',
    memo?: string,
    funds?: Coin[],
  ) => Promise<ExecuteResult>
}
export class MarsSwapperBaseClient
  extends MarsSwapperBaseQueryClient
  implements MarsSwapperBaseInterface
{
  client: SigningCosmWasmClient
  sender: string
  contractAddress: string

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress)
    this.client = client
    this.sender = sender
    this.contractAddress = contractAddress
    this.updateAdmin = this.updateAdmin.bind(this)
    this.setRoute = this.setRoute.bind(this)
    this.swapExactIn = this.swapExactIn.bind(this)
    this.transferResult = this.transferResult.bind(this)
  }

  updateAdmin = async (
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        update_admin: {},
      },
      fee,
      memo,
      funds,
    )
  }
  setRoute = async (
    {
      denomIn,
      denomOut,
      route,
    }: {
      denomIn: string
      denomOut: string
      route: Empty
    },
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        set_route: {
          denom_in: denomIn,
          denom_out: denomOut,
          route,
        },
      },
      fee,
      memo,
      funds,
    )
  }
  swapExactIn = async (
    {
      coinIn,
      denomOut,
      slippage,
    }: {
      coinIn: Coin
      denomOut: string
      slippage: Decimal
    },
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        swap_exact_in: {
          coin_in: coinIn,
          denom_out: denomOut,
          slippage,
        },
      },
      fee,
      memo,
      funds,
    )
  }
  transferResult = async (
    {
      denomIn,
      denomOut,
      recipient,
    }: {
      denomIn: string
      denomOut: string
      recipient: Addr
    },
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        transfer_result: {
          denom_in: denomIn,
          denom_out: denomOut,
          recipient,
        },
      },
      fee,
      memo,
      funds,
    )
  }
}
