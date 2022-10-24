use cosmwasm_std::{Empty, Addr, Coin, coins};
use cw_multi_test::{App, ContractWrapper, Contract};
use crate::{execute, instantiate, query, multitest::CountingContract};
use counting_contract_0_1_0::multitest::CountingContract as Counting_Contract_0_1_0;


fn counting_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}

#[test]
fn query_value() {
    let mut app = App::default();
    let contract_id = app.store_code(counting_contract());
    let sender = Addr::unchecked("sender");
    let contract = CountingContract::instantiate(
        &mut app, 
        contract_id, 
        &sender, 
        None,
        "Counting Contract",
        Coin::new(10, "atom")
        )
    .unwrap();
    
    let resp = contract.query_value(&app).unwrap();
    assert_eq!(resp.value, 0);
}

#[test]
fn donate() {
    let mut app = App::default();
    let contract_id = app.store_code(counting_contract());
    let sender = Addr::unchecked("sender");
    let contract = CountingContract::instantiate(
        &mut app, 
        contract_id, 
        &sender, 
        None,
        "Counting Contract",
        Coin::new(10, "atom")
        )
    .unwrap();

    contract.donate(&mut app, &sender, &[]).unwrap();
    let resp = contract.query_value(&app).unwrap();
    assert_eq!(resp.value, 0);
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
    let contract = CountingContract::instantiate(
        &mut app, 
        contract_id, 
        &sender, 
        None,
        "Counting Contract",
        Coin::new(10, "atom")
        )
    .unwrap();

    contract.donate(&mut app, &sender, &coins(10, "atom")).unwrap();
    let resp = contract.query_value(&app).unwrap();
    assert_eq!(resp.value, 1);

    assert_eq!(
        app.wrap().query_all_balances(contract.addr()).unwrap(),
        coins(10, "atom")
    )
}

#[test]
fn withdraw() {
    let owner = Addr::unchecked("owner");
    let sender1 = Addr::unchecked("sender1");
    let sender2 = Addr::unchecked("sender2");
    let mut app = App::new( |router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender1, coins(10, "atom"))
            .unwrap();
        router
            .bank
            .init_balance(storage, &sender2, coins(5, "atom"))
            .unwrap();
    });
    let contract_id = app.store_code(counting_contract());
    let contract = CountingContract::instantiate(
        &mut app, 
        contract_id, 
        &owner, 
        None,
        "Counting Contract",
        Coin::new(10, "atom")
        )
    .unwrap();

    contract.donate(&mut app, &sender1, &coins(10, "atom")).unwrap();
    contract.donate(&mut app, &sender2, &coins(5, "atom")).unwrap();

    contract.withdraw(&mut app, &owner).unwrap();
    assert_eq!(
        app.wrap().query_all_balances(contract.addr()).unwrap(),
        vec![]
    );
    assert_eq!(app.wrap().query_all_balances(sender1).unwrap(), vec![]);
    assert_eq!(app.wrap().query_all_balances(sender2).unwrap(), vec![]);
}
#[test]
fn migration() {
    let owner = Addr::unchecked("owner");
    let admin = Addr::unchecked("admin");
    let sender = Addr::unchecked("sender");

    let mut app = App::new( |router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender1, coins(10, "atom"))
            .unwrap();
    });

    let old_code_id = Counting_Contract_0_1_0::store_code(&mut app);
}