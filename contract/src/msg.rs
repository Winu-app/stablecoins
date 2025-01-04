use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct InstantiateMsg {
    pub initial_supply: u128,
    pub peg_price: u128,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(u128)]
    GetSupply {},
    #[returns(u128)]
    GetPegPrice {},
    #[returns(u128)]
    GetWithdrawalLimit {},
}

#[cw_serde]
pub enum ExecuteMsg {
    Mint { amount: u128 },
    Burn { amount: u128 },
    UpdatePegPrice { peg_price: u128 },
    Deposit { amount: u128 },
    Withdraw { amount: u128 },
    UpdateWithdrawalLimit { limit: u128 },
    SynchronizeWithMain {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct User {
    pub wallet: Addr,
    pub balance: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Exchange {
    pub name: String,
    pub balance: Uint128,
    pub withdrawal_limit: Uint128,
}
