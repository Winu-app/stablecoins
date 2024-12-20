use cosmwasm_std::Addr;
use cw_storage_plus::{Item};

pub const OWNER: Item<Addr> = Item::new("owner");
pub const TOTAL_SUPPLY: Item<u128> = Item::new("total_supply");
pub const PEG_PRICE: Item<u128> = Item::new("peg_price");