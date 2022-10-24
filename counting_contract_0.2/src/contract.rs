use crate::state::{STATE, OWNER};
// use crate::
use cosmwasm_std::{DepsMut, Response, StdResult, Coin, MessageInfo};
use crate::state::{State, OWNER, STATE};

pub fn instantiate(deps: DepsMut, info: MessageInfo, counter: u64, minimal_donation: Coin) -> StdResult<Response> {
    STATE.save(deps.storage, &State {count: 0, minimal_donation: msg.minimal_donation,});
    OWNER.save(deps.storage, &info.sender)?;
    Ok(Response::new())
}

pub mod execute {
    use cosmwasm_std::{StdResult, Response, DepsMut, MessageInfo, Env, BankMsg};

    use crate::{state::{STATE, OWNER}, error::ContractError};

    pub fn donate(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        let mut state = STATE.load(deps.storage)?;
        let minimal_donation = MINIMAL_DONATION.load(deps.storage)?;

        if info.funds.iter().any(|coin| {
            coin.denom == state.minimal_donation.denom && coin.amount >= state.minimal_donation.amount
        }) {
            state.counter += 1;
            STATE.save(deps.storage, &counter)?;
        }

        let resp = Response::new()
        .add_attribute("action", "donate")
        .add_attribute("sender", info.sender)
        .add_attribute("counter", state.counter.to_string());
        Ok(resp)
    }
    pub fn reset(deps: DepsMut) -> StdResult<Response> {
        COUNTER.save(deps.storage, &0)?;

        let resp = Response::new()
        .add_attribute("action", "reset_counter")
        .add_attribute("counter", 0.to_string());
        Ok(resp)
    }
    pub fn withdraw(deps: DepsMut, info: MessageInfo, env: Env) -> Result<Response, ContractError> {
        let owner = OWNER.load(deps.storage)?;
        if info.sender != owner {
            return Err(ContractError::Unauthorized { owner: owner.to_string() });
        }
        //queries contract state and token value
        let balance = deps.querier.query_all_balances(&env.contract.address)?;
        let bank_msg = BankMsg::Send { to_address: info.sender.to_string(), amount: balance };

        let resp = Response::new()
            .add_message(bank_msg)
            .add_attribute("action", "withdraw")
            .add_attribute("sender", info.sender.to_string());

        Ok(resp)

    }
}

pub mod query {
    use crate::msg::ValueResp;
    use crate::state::STATE;
    use cosmwasm_std::{Deps, StdResult};

    pub fn value(deps: Deps) -> StdResult<ValueResp> {
        let value = STATE.load(deps.storage)?.counter;
        Ok(ValueResp { value })
    }
}
