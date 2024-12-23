use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_json_binary};
use crate::error::ContractError;
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::state::{OWNER, TOTAL_SUPPLY, PEG_PRICE, WITHDRAWAL_LIMIT};
use crate::helpers::validate_positive_amount;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    OWNER.save(deps.storage, &info.sender)?;
    TOTAL_SUPPLY.save(deps.storage, &msg.initial_supply)?;
    PEG_PRICE.save(deps.storage, &msg.peg_price)?;
    WITHDRAWAL_LIMIT.save(deps.storage, &0u128)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint { amount } => execute_mint(deps, info, amount),
        ExecuteMsg::Burn { amount } => execute_burn(deps, info, amount),
        ExecuteMsg::UpdatePegPrice { peg_price } => execute_update_peg_price(deps, info, peg_price),
        ExecuteMsg::Deposit { amount } => execute_deposit(deps, info, amount),
        ExecuteMsg::Withdraw { amount } => execute_withdraw(deps, info, amount),
        ExecuteMsg::UpdateWithdrawalLimit { limit } => execute_update_withdrawal_limit(deps, info, limit),
    }
}

fn execute_mint(
    deps: DepsMut,
    info: MessageInfo,
    amount: u128,
) -> Result<Response, ContractError> {
    validate_positive_amount(amount)?;
    let owner = OWNER.load(deps.storage)?;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    TOTAL_SUPPLY.update(deps.storage, |supply| -> StdResult<_> {
        Ok(supply + amount)
    })?;

    Ok(Response::new().add_attribute("action", "mint").add_attribute("amount", amount.to_string()))
}

fn execute_burn(
    deps: DepsMut,
    info: MessageInfo,
    amount: u128,
) -> Result<Response, ContractError> {
    validate_positive_amount(amount)?;
    let owner = OWNER.load(deps.storage)?;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    TOTAL_SUPPLY.update(deps.storage, |supply| -> Result<_, ContractError> {
        if supply < amount {
            return Err(ContractError::InsufficientFunds {});
        }
        Ok(supply - amount)
    })?;

    Ok(Response::new().add_attribute("action", "burn").add_attribute("amount", amount.to_string()))
}

fn execute_update_peg_price(
    deps: DepsMut,
    info: MessageInfo,
    peg_price: u128,
) -> Result<Response, ContractError> {
    validate_positive_amount(peg_price)?;
    let owner = OWNER.load(deps.storage)?;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    PEG_PRICE.save(deps.storage, &peg_price)?;

    Ok(Response::new().add_attribute("action", "update_peg_price").add_attribute("peg_price", peg_price.to_string()))
}

fn execute_deposit(
    deps: DepsMut,
    _info: MessageInfo,
    amount: u128,
) -> Result<Response, ContractError> {
    validate_positive_amount(amount)?;

    TOTAL_SUPPLY.update(deps.storage, |supply| -> StdResult<_> {
        Ok(supply + amount)
    })?;

    Ok(Response::new().add_attribute("action", "deposit").add_attribute("amount", amount.to_string()))
}

fn execute_withdraw(
    deps: DepsMut,
    _info: MessageInfo,
    amount: u128,
) -> Result<Response, ContractError> {
    validate_positive_amount(amount)?;
    let withdrawal_limit = WITHDRAWAL_LIMIT.load(deps.storage)?;

    if amount > withdrawal_limit {
        return Err(ContractError::WithdrawalLimitExceeded {});
    }

    TOTAL_SUPPLY.update(deps.storage, |supply| -> Result<_, ContractError> {
        if supply < amount {
            return Err(ContractError::InsufficientFunds {});
        }
        Ok(supply - amount)
    })?;

    Ok(Response::new().add_attribute("action", "withdraw").add_attribute("amount", amount.to_string()))
}

fn execute_update_withdrawal_limit(
    deps: DepsMut,
    info: MessageInfo,
    limit: u128,
) -> Result<Response, ContractError> {
    validate_positive_amount(limit)?;
    let owner = OWNER.load(deps.storage)?;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    WITHDRAWAL_LIMIT.save(deps.storage, &limit)?;

    Ok(Response::new().add_attribute("action", "update_withdrawal_limit").add_attribute("limit", limit.to_string()))
}

#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetSupply {} => to_json_binary(&TOTAL_SUPPLY.load(deps.storage)?),
        QueryMsg::GetPegPrice {} => to_json_binary(&PEG_PRICE.load(deps.storage)?),
        QueryMsg::GetWithdrawalLimit {} => to_json_binary(&WITHDRAWAL_LIMIT.load(deps.storage)?),
    }
}
