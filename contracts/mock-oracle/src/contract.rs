use cosmwasm_std::{
    to_binary, to_vec, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult,
};

use crate::msg::{CoinPrice, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::COIN_PRICE;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    for item in msg.coins {
        COIN_PRICE.save(deps.storage, item.denom, &item.price)?
    }
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::ChangePrice(item) => change_price(deps, item),
    }
}

fn change_price(deps: DepsMut, coin: CoinPrice) -> StdResult<Response> {
    COIN_PRICE.save(deps.storage, coin.denom, &coin.price)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AssetPrice { denom } => to_binary(&query_asset_price(deps, denom)?),
        _ => Err(StdError::generic_err(format!(
            "[mock] unimplemented query: {}",
            String::from_utf8(to_vec(&msg)?)?
        ))),
    }
}

// TODO: After mars-core bumps to the next version https://crates.io/crates/mars-core (currently 1.0.0)
// should update this mock to return MarsDecimal:  https://github.com/mars-protocol/mars-core/blob/master/packages/mars-core/src/math/decimal.rs
fn query_asset_price(deps: Deps, denom: String) -> StdResult<Decimal> {
    COIN_PRICE.load(deps.storage, denom)
}