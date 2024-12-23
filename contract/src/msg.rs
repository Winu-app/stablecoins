use cosmwasm_schema::{cw_serde, QueryResponses};

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
}