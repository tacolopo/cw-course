use cw_storage_plus::Item;

//an item is a type accessing a single object which may exist in the blockchain storage
pub const COUNTER: Item<u64> = Item::new("counter");
