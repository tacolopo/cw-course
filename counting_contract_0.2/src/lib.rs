use crate::msg::QueryMsg;
use cosmwasm_std::{
    to_binary, Empty, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use error::ContractError;
use msg::{ExecuteMsg, InstantiateMsg};

mod contract;
pub mod msg;
mod state;
pub mod error;
#[cfg(any(test, feature = "tests"))]
pub mod multitest;

//hides entry point when library feature is enabled
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    contract::instantiate(deps, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> StdResult<Response> {
    contract::migrate(deps)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use msg::ExecuteMsg::*;

    match msg {
        Donate {} => contract::execute::donate(deps, info).map_err(ContractError::Std),
        Withdraw {} => contract::execute::withdraw(deps, info, env),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use msg::QueryMsg::*;
    match msg {
        Value {} => to_binary(&contract::query::value(deps)?),
    }
}
