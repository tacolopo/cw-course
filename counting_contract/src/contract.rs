use crate::state::{COUNTER, MINIMAL_DONATION};
// use crate::
use cosmwasm_std::{DepsMut, Response, StdResult, Coin};

pub fn instantiate(deps: DepsMut, counter: u64, minimal_donation: Coin) -> StdResult<Response> {
    COUNTER.save(deps.storage, &counter)?;
    MINIMAL_DONATION.save(deps.storage, &minimal_donation)?;
    Ok(Response::new())
}

pub mod execute {
    use cosmwasm_std::{StdResult, Response, DepsMut, MessageInfo};

    use crate::state::{COUNTER, MINIMAL_DONATION};

    pub fn donate(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        let mut counter = COUNTER.load(deps.storage)?;
        let minimal_donation = MINIMAL_DONATION.load(deps.storage)?;

        if info.funds.iter().any(|coin| {
            coin.denom == minimal_donation.denom && coin.amount >= minimal_donation.amount
        }) {
            counter += 1;
            COUNTER.save(deps.storage, &counter)?;
        }

        let resp = Response::new()
        .add_attribute("action", "donate")
        .add_attribute("sender", info.sender)
        .add_attribute("counter", counter.to_string());
        Ok(resp)
    }
    pub fn reset(deps: DepsMut) -> StdResult<Response> {
        COUNTER.save(deps.storage, &0)?;

        let resp = Response::new()
        .add_attribute("action", "reset_counter")
        .add_attribute("counter", 0.to_string());
        Ok(resp)
    }
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
