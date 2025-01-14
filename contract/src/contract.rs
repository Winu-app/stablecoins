use cosmwasm_std::{entry_point, to_json_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use crate::error::ContractError;
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::state::{BALANCES, EXCHANGES, OWNER, PEG_PRICE, TOTAL_SUPPLY, WITHDRAWAL_LIMIT};
use crate::helpers::validate_positive_amount;

use crate::msg::Exchange;

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
    env: Env,
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
        ExecuteMsg::CorrectTotalSupply { desired_total_supply } => execute_correct_total_supply(deps,info, desired_total_supply),//TODO
        ExecuteMsg::CreateNewExchange { name, withdrawal_limit, initial_funds } => execute_create_new_exchange(deps, info,name,initial_funds,withdrawal_limit),
        ExecuteMsg::BuyFromExchange { exchange_address,amount } => execute_buy_from_exchange(deps, info, exchange_address, amount),
        ExecuteMsg::SellToExchange { exchange_address,amount } => execute_sell_to_exchange(deps, info, exchange_address, amount),
        
        ExecuteMsg::TransferFunds { recipient, amount } => execute_transfer_funds(deps, env, info, recipient, amount),
    }
}

fn execute_correct_total_supply(
    deps: DepsMut, 
    info: MessageInfo,
    desired_total_supply: u128
)-> Result<Response, ContractError>{
    validate_positive_amount(desired_total_supply)?;
    let owner = OWNER.load(deps.storage)?;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    let difference = desired_total_supply - TOTAL_SUPPLY.load(deps.storage)?;

    if difference > 0 {
        TOTAL_SUPPLY.update(deps.storage, |supply| -> StdResult<_> {
            Ok(supply + difference)
        })?;
    } else {
        TOTAL_SUPPLY.update(deps.storage, |supply| -> Result<_, ContractError> {
            if supply < desired_total_supply {
                return Err(ContractError::InsufficientFunds {});
            }
            Ok(supply - difference)
        })?;
    }

    Ok(Response::new().add_attribute("action", "create exchange").add_attribute("correct supply", desired_total_supply.to_string()))
}


fn execute_create_new_exchange(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
    initial_funds: u128,
    withdrawal_limit: u128,
)-> Result<Response, ContractError> {
    validate_positive_amount(initial_funds)?;

    if EXCHANGES.has(deps.storage, &info.sender) {
        return Err(ContractError::ExchangeAlreadyExists {});
    }

    let exchange = &Exchange {
        name,
        balance: initial_funds,
        withdrawal_limit,
        owner: info.sender.clone(),
    };

    EXCHANGES.save(deps.storage, &info.sender, exchange)?;
    Ok(Response::new().add_attribute("action", "create exchange").add_attribute("initial_funds", initial_funds.to_string()))
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


fn execute_buy_from_exchange(deps:DepsMut, info:MessageInfo, exchange_address:Addr, amount: u128)-> Result<Response, ContractError>{
    validate_positive_amount(amount)?;
    let exchange = EXCHANGES.load(deps.storage, &exchange_address)?;

    if amount > exchange.balance {
        return Err(ContractError::InsufficientFunds {});
    }

    EXCHANGES.update(deps.storage, &exchange_address, |maybe_exchange| {
        let mut exchange = maybe_exchange.ok_or(ContractError::ExchangeNotFound {})?;
        if amount > exchange.balance {
            return Err(ContractError::InsufficientFunds {});
        }
        exchange.balance -= amount;
        Ok(exchange)
    })?;

    BALANCES.update(deps.storage, &info.sender, |balance| -> Result<_, ContractError> {
        Ok(balance.unwrap_or(0) + amount)
    })?;

    Ok(Response::new().add_attribute("action", "buy_coins").add_attribute("amount", amount.to_string()))
}

fn execute_transfer_funds(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Vec<Coin>,
) -> Result<Response, ContractError>{

    if amount.is_empty() {
        return Err(ContractError::InvalidAmount {});
    }
    if amount.is_empty() {
        return Err(ContractError::InvalidAmount {});
    }
    let send_msg = BankMsg::Send {
        to_address: recipient.clone(),
        amount: amount.clone(),
    };

    let response = Response::new()
        .add_message(CosmosMsg::Bank(send_msg))
        .add_attribute("action", "send_payment")
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("recipient", recipient)
        .add_attribute("amount", format!("{:?}", amount));

    Ok(response)
}

fn execute_sell_to_exchange(deps:DepsMut, info:MessageInfo, exchange_address:Addr, amount: u128)-> Result<Response, ContractError>{
    validate_positive_amount(amount)?;

    let user_balance = BALANCES.may_load(deps.storage, &info.sender)?.unwrap_or(0);

    if amount > user_balance {
        return Err(ContractError::InsufficientFunds {});
    }
 
    EXCHANGES.update(deps.storage, &exchange_address, |maybe_exchange| {
        let mut exchange = maybe_exchange.ok_or(ContractError::ExchangeNotFound {})?;
        if amount > exchange.balance {
            return Err(ContractError::InsufficientFunds {});
        }
        exchange.balance += amount;
        Ok(exchange)
    })?;

    BALANCES.update(deps.storage, &info.sender, |balance| -> Result<_, ContractError> {
        Ok(balance.unwrap_or(0) - amount)
    })?;

    Ok(Response::new().add_attribute("action", "sell_coins").add_attribute("amount", amount.to_string()))
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
        QueryMsg::GetExchange { owner } => query_get_exchange(deps, owner),
        QueryMsg::GetUserBalance { address } => query_get_user_balance(deps, address),
        QueryMsg::GetExchangeBalance { address } => query_get_exchange_balance(deps, address),
    }
}

pub fn query_get_exchange_balance(deps: Deps, address: Addr) -> StdResult<Binary> {
    let balance = EXCHANGES.load(deps.storage, &address)?.balance;
    to_json_binary(&balance)
}

pub fn query_get_user_balance(deps: Deps, address: Addr) -> StdResult<Binary> {
    let balance = BALANCES.may_load(deps.storage, &address)?.unwrap_or(0);
    to_json_binary(&balance)
}

pub fn query_get_exchange(deps: Deps, owner: Addr) -> StdResult<Binary> {
    
    let exchange = EXCHANGES.load(deps.storage, &owner)?;
    to_json_binary(&exchange)
}