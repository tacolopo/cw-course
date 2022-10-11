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
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
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

//cfg compiles code only if predicate passed is true. "our test would not be unnecessarily sitting in the final binary"
#[cfg(test)]
mod test {
    use cosmwasm_std::{Addr, Empty};
    use cw_multi_test::{Contract, ContractWrapper};
    use cw_multi_test::{App, Executor};
    use crate::msg::{ValueResp, QueryMsg};

    use crate::{execute, instantiate, query};

    fn counting_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    #[test]
    fn query_value() {
        //default app instance "it is the blockchain simulator"
        let mut app = App::default();
        //mimicks test storing code on the blockchain
        let contract_id = app.store_code(counting_contract());
        //mimicks instantiating contract on chain
        let contract_addr = app
            .instantiate_contract(contract_id, Addr::unchecked("sender"), &Empty {}, &[], "Counting Contract", None).unwrap();

        //wrap converts app object to a temporary QuerierWrapper, allowing us to query the chain
        let resp: ValueResp = app.wrap().query_wasm_smart(contract_addr, &QueryMsg::Value {  }).unwrap();

        assert_eq!(resp, ValueResp{value:0});
}
}