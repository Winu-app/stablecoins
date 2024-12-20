use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,to_json_binary,StdError};
use crate::error::ContractError;
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::state::{OWNER, TOTAL_SUPPLY, PEG_PRICE};
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

    TOTAL_SUPPLY.update(deps.storage, |supply| -> StdResult<_> {
        if supply < amount {
            return Err(StdError::generic_err("Insufficient funds for burn operation"));
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

#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetSupply {} => to_json_binary(&TOTAL_SUPPLY.load(deps.storage)?),
        QueryMsg::GetPegPrice {} => to_json_binary(&PEG_PRICE.load(deps.storage)?),
    }
}