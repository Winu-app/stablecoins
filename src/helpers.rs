use cosmwasm_std::StdError;
use crate::error::ContractError;

pub fn validate_positive_amount(amount: u128) -> Result<(), ContractError> {
    if amount == 0 {
        return Err(ContractError::Std(StdError::generic_err("Amount must be greater than zero")));
    }
    Ok(())
}
