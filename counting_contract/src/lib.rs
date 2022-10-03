use cosmwasm_std::{entry_point, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, StdResult, Binary, to_binary};
use crate::msg::QueryMsg;
use crate::msg::QueryMsg::{Value};

mod contract;
pub mod msg;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> Result<Response, StdError> {
    Ok(Response::new())
}

#[entry_point]
pub fn query(
    _deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        Value {} => to_binary(&contract::query::value()),
    }
}