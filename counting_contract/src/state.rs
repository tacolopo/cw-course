use cosmwasm_std::{Coin, Addr};
use cw_storage_plus::Item;

//an item is a type accessing a single object which may exist in the blockchain storage
pub const COUNTER: Item<u64> = Item::new("counter");
pub const MINIMAL_DONATION: Item<Coin> = Item::new("minimal_donation");
// store authorized withdrawer
pub const OWNER: Item<Addr> = Item::new("owner");
