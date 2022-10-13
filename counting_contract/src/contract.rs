use crate::state::COUNTER;
use cosmwasm_std::{DepsMut, Response, StdResult};

pub fn instantiate(deps: DepsMut) -> StdResult<Response> {
    COUNTER.save(deps.storage, &0)?;
    Ok(Response::new())
}

pub mod query {
    use crate::msg::ValueResp;
    use crate::state::COUNTER;
    use cosmwasm_std::{Deps, StdResult};

    pub fn value(deps: Deps) -> StdResult<ValueResp> {
        let value = COUNTER.load(deps.storage)?;
        Ok(ValueResp { value })
    }
}

pub mod exec {
    use cosmwasm_std::{StdResult, Response, DepsMut, MessageInfo};

    use crate::state::COUNTER;

    pub fn poke(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        let value = COUNTER.load(deps.storage)? + 1;
        COUNTER.save(deps.storage, &value)?;

        let resp = Response::new().add_attribute("action", "poke").add_attribute("sender", info.sender).add_attribute("counter", value.to_string());
        Ok(resp)
    }
}