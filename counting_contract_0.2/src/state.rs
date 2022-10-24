use cosmwasm_std::{Coin, Addr};
use cw_storage_plus::Item;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct State {
    pub counter: u64,
    pub minimal_donation: Coin,
}

pub const STATE: Item<State> = Item::new("state");
pub const OWNER: Item<Addr> = Item::new("owner");
