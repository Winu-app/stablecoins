use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Insufficient Funds")]
    InsufficientFunds {},

    #[error("Exchange Not Found")]
    ExchangeNotFound {},

    #[error("Amount to send cannot be empty")]
    InvalidAmount {},

    #[error("Withdrawal Limit Exceeded")]
    WithdrawalLimitExceeded {},

    #[error("Exchange already exists for this owner.")]
    ExchangeAlreadyExists {},
}