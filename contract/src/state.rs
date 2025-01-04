use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

pub const OWNER: Item<Addr> = Item::new("owner");
pub const TOTAL_SUPPLY: Item<u128> = Item::new("total_supply");
pub const PEG_PRICE: Item<u128> = Item::new("peg_price");
pub const WITHDRAWAL_LIMIT: Item<u128> = Item::new("withdrawal_limit");

pub const EXCHANGE_BALANCES: Map<&Addr, Uint128> = Map::new("exchange_balances");