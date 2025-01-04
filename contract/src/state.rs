use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

use crate::msg::Exchange;

pub const OWNER: Item<Addr> = Item::new("owner");
pub const TOTAL_SUPPLY: Item<u128> = Item::new("total_supply");
pub const PEG_PRICE: Item<u128> = Item::new("peg_price");
pub const WITHDRAWAL_LIMIT: Item<u128> = Item::new("withdrawal_limit");

pub const EXCHANGES: Map<&Addr, Exchange> = Map::new("exchanges");