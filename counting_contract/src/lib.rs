use crate::msg::QueryMsg;
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use error::ContractError;
use msg::{ExecuteMsg, InstantiateMsg};

mod contract;
pub mod msg;
mod state;
mod error;

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

//cfg compiles code only if predicate passed is true. "our test would not be unnecessarily sitting in the final binary"
#[cfg(test)]
mod test {
    use crate::error::ContractError;
    use crate::msg::{ExecuteMsg, QueryMsg, ValueResp, InstantiateMsg};
    use cosmwasm_std::{Addr, Empty, coin, coins};
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
                &InstantiateMsg {
                    counter: 0,
                    minimal_donation: coin(10, "atom"),
                },
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
    fn donate() {
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
                &InstantiateMsg {
                    counter: 0,
                    minimal_donation: coin(10, "atom")
                },
                &[],
                "Counting Contract",
                None,
            )
            .unwrap();

        app.execute_contract(sender, contract_addr.clone(), &ExecuteMsg::Donate {}, &[])
            .unwrap();
        //wrap converts app object to a temporary QuerierWrapper, allowing us to query the chain
        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 0 });
    }
    #[test]
    fn donate_with_funds() {
        let sender = Addr::unchecked("sender");
        let mut app = App::new( |router, _api, storage| {
            router
                .bank
                .init_balance(storage, &sender, coins(10, "atom"))
                .unwrap()
        });
        let contract_id = app.store_code(counting_contract());
        let contract_addr = app
            .instantiate_contract(
                contract_id,
                sender.clone(),
                &InstantiateMsg {
                    counter: 0,
                    minimal_donation: coin(10, "atom")
                },
                &[],
                "Counting Contract",
                None,
            )
            .unwrap();

        app.execute_contract(sender, contract_addr.clone(), &ExecuteMsg::Donate {}, &coins(10, "atom"))
            .unwrap();
        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 1 });
    }
    #[test]
    fn withdraw() {
        let owner = Addr::unchecked("owner");
        let sender = Addr::unchecked("sender");

        let mut app = App::new( |router, _api, storage| {
            router
                .bank
                .init_balance(storage, &sender, coins(10, "atom"))
                .unwrap()
        });
        
        let contract_id = app.store_code(counting_contract());
        let contract_addr = app
        .instantiate_contract(
            contract_id,
            owner.clone(),
            &InstantiateMsg {
                counter: 0,
                minimal_donation: coin(10, "atom")
            },
            &[],
            "Counting Contract",
            None,
        )
        .unwrap();
        app.execute_contract(
            sender.clone(), 
            contract_addr.clone(), 
            &ExecuteMsg::Donate {}, 
            &coins(10, "atom")
        )
        .unwrap();
        app.execute_contract(
            owner.clone(), 
            contract_addr.clone(), 
            &ExecuteMsg::Withdraw {  }, 
            &[]
        )
        .unwrap();

        assert_eq!(
            app.wrap().query_all_balances(owner).unwrap(),
            coins(10, "atom")
        );
    }
    #[test]
    fn unauthorized_withdraw() {
        let owner = Addr::unchecked("owner");
        let member = Addr::unchecked("member");
        let mut app = App::default();
        let contract_id = app.store_code(counting_contract());
        let contract_addr = app
        .instantiate_contract(
            contract_id,
            owner.clone(),
            &InstantiateMsg {
                counter: 0,
                minimal_donation: coin(10, "atom")
            },
            &[],
            "Counting Contract",
            None,
        )
        .unwrap();

        let err = app
        .execute_contract(member, contract_addr, &ExecuteMsg::Withdraw {  }, &[])
        .unwrap_err();

        assert_eq!(
            ContractError::Unauthorized { owner: owner.into() },
            err.downcast().unwrap()
        );
    }
}
