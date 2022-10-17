use crate::msg::QueryMsg;
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use error::ContractError;
use msg::{ExecuteMsg, InstantiateMsg};

mod contract;
pub mod msg;
mod state;
pub mod error;
#[cfg(test)]
pub mod multitest;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    contract::instantiate(deps, info, msg.counter, msg.minimal_donation)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use msg::ExecuteMsg::*;

    match msg {
        Donate {} => contract::execute::donate(deps, info).map_err(ContractError::Std),
        Reset {} => contract::execute::reset(deps).map_err(ContractError::Std),
        Withdraw {} => contract::execute::withdraw(deps, info, env),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use msg::QueryMsg::*;
    match msg {
        Value {} => to_binary(&contract::query::value(deps)?),
    }
}
