use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};
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
    #[returns(Exchange)]
    GetExchange { owner: Addr },
    #[returns(u128)]
    GetUserBalance{address:Addr},
    #[returns(u128)]
    GetExchangeBalance{address:Addr},
    #[returns(Vec<Exchange>)]
    GetAllExchanges{}
}

#[cw_serde]
pub enum ExecuteMsg {
    Mint { amount: u128 },
    Burn { amount: u128 },
    UpdatePegPrice { peg_price: u128 },
    Deposit { amount: u128 },
    Withdraw { amount: u128 },
    UpdateWithdrawalLimit { limit: u128 },
    CorrectTotalSupply {desired_total_supply: u128},
    CreateNewExchange { name: String, withdrawal_limit: u128, initial_funds: u128 },
    BuyFromExchange { exchange_address: Addr, amount: u128 },
    SellToExchange { exchange_address: Addr, amount: u128 },
    TransferFunds { recipient: String, amount: Vec<Coin> }
}




#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, schemars::JsonSchema)]
pub struct Exchange {
    pub name: String,
    pub balance: u128,
    pub withdrawal_limit: u128,
    pub owner: Addr,
}
