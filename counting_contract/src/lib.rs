use crate::msg::QueryMsg;
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError,
    StdResult,
};
use msg::ExecuteMsg;


mod contract;
pub mod msg;
mod state;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> Result<Response, StdError> {
    contract::instantiate(deps)
}

#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    use msg::ExecuteMsg::*;

    match msg {
        Poke {} => contract::exec::poke(deps, info),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use msg::QueryMsg::*;
    match msg {
        Value {} => to_binary(&contract::query::value(deps)?),
    }
}

//cfg compiles code only if predicate passed is true. "our test would not be unnecessarily sitting in the final binary"
#[cfg(test)]
mod test {
    use crate::msg::{QueryMsg, ValueResp, ExecuteMsg};
    use cosmwasm_std::{Addr, Empty};
    use cw_multi_test::{App, Executor};
    use cw_multi_test::{Contract, ContractWrapper};

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
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &Empty {},
                &[],
                "Counting Contract",
                None,
            )
            .unwrap();

        //wrap converts app object to a temporary QuerierWrapper, allowing us to query the chain
        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 0 });
    }

    #[test]
    fn poke() {
        //default app instance "it is the blockchain simulator"
        let mut app = App::default();
        let sender = Addr::unchecked("sender");
        //mimicks test storing code on the blockchain
        let contract_id = app.store_code(counting_contract());
        //mimicks instantiating contract on chain
        let contract_addr = app
            .instantiate_contract(
                contract_id,
                sender.clone(),
                &Empty {},
                &[],
                "Counting Contract",
                None,
            )
            .unwrap();

        app.execute_contract(sender, contract_addr.clone(), &ExecuteMsg::Poke {  }, &[]).unwrap();
        //wrap converts app object to a temporary QuerierWrapper, allowing us to query the chain
        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 1 });
    }
}
