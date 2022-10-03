use cosmwasm_std::{entry_point, DepsMut, Empty, Env, MessageInfo, Response, StdError};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> Result<Response, StdError> {
    Ok(Response::new())
}