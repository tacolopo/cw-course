use cosmwasm_std::{Addr, Coin, StdResult, StdError, Empty};
use crate::{InstantiateMsg, msg::{ValueResp, QueryMsg, ExecuteMsg}, error::ContractError};
use crate::{execute, instantiate, query, migrate};
use cw_multi_test::{App, Executor, ContractWrapper};
#[cfg(test)]
mod tests;
pub struct CountingContract(Addr);

impl CountingContract {
    pub fn addr(&self) -> &Addr {
        &self.0
    }
    pub fn store_code(app: &mut App) -> u64 {
        let contract = ContractWrapper::new(execute, instantiate, query).with_migrate(migrate);
        app.store_code(Box::new(contract))
    }
    #[track_caller]
    pub fn instantiate(
        app: &mut App, 
        code_id: u64, 
        sender: &Addr, 
        admin: Option<&Addr>,
        label: &str, 
        minimal_donation: Coin
    ) -> StdResult<CountingContract> {
        app.instantiate_contract(
            code_id, 
            sender.clone(), 
            &InstantiateMsg { minimal_donation, counter: 0 }, 
            &[], 
            label,
            admin.map(Addr::to_string),
        )
        .map_err(|err| err.downcast().unwrap())
        .map(CountingContract)
    }

    #[track_caller]
    pub fn migrate(app: &mut App, sender: &Addr, contract: &Addr, code_id: u64) -> StdResult<Self> {
        app.migrate_contract(sender.clone(), contract.clone(), &Empty {}, code_id)
            .map_err(|err| err.downcast::<StdError>().unwrap())?;
        Ok(CountingContract(contract.clone()))
        }

    #[track_caller]
    pub fn donate(&self, app: &mut App, sender: &Addr, funds: &[Coin]) -> Result<(), ContractError> {
        app.execute_contract(sender.clone(), self.0.clone(), &ExecuteMsg::Donate{}, funds)
        .map_err(|err| err.downcast::<ContractError>().unwrap())?;

        Ok(())
    }

    #[track_caller]
    pub fn withdraw(&self, app: &mut App, sender: &Addr) -> Result<(), ContractError> {
        app.execute_contract(sender.clone(), self.0.clone(), &ExecuteMsg::Withdraw {  }, &[])
        .map_err(|err| err.downcast::<ContractError>().unwrap())?;

        Ok(())
    }

    pub fn query_value(&self, app: &App) -> StdResult<ValueResp> {
        app.wrap().query_wasm_smart(self.0.clone(), &QueryMsg::Value {  },)
    }
}