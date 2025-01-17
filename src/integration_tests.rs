#[cfg(test)]
mod integration_tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_binary, Addr};
    use crate::contract::{instantiate, execute, query};
    use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
    use crate::state::{OWNER, TOTAL_SUPPLY, PEG_PRICE, WITHDRAWAL_LIMIT};
    use crate::error::ContractError;

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            initial_supply: 1_000_000,
            peg_price: 1_000,
        };

        let info = mock_info("creator", &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        assert_eq!(res.messages.len(), 0);
        assert_eq!(OWNER.load(&deps.storage).unwrap(), Addr::unchecked("creator"));
        assert_eq!(TOTAL_SUPPLY.load(&deps.storage).unwrap(), 1_000_000);
        assert_eq!(PEG_PRICE.load(&deps.storage).unwrap(), 1_000);
    }

    #[test]
    fn test_withdrawal_within_limit() {
        let mut deps = mock_dependencies();

        let init_msg = InstantiateMsg {
            initial_supply: 1_000_000,
            peg_price: 1_000,
        };
        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg).unwrap();

        // Update withdrawal limit to 500_000
        let update_limit_msg = ExecuteMsg::UpdateWithdrawalLimit { limit: 500_000 };
        execute(deps.as_mut(), mock_env(), info.clone(), update_limit_msg).unwrap();

        // Withdraw 400_000, within limit
        let withdraw_msg = ExecuteMsg::Withdraw { amount: 400_000 };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), withdraw_msg).unwrap();

        assert_eq!(res.messages.len(), 0);
        assert_eq!(TOTAL_SUPPLY.load(&deps.storage).unwrap(), 600_000);
    }

    #[test]
    fn test_withdrawal_exceeding_limit() {
        let mut deps = mock_dependencies();

        let init_msg = InstantiateMsg {
            initial_supply: 1_000_000,
            peg_price: 1_000,
        };
        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg).unwrap();

        // Update withdrawal limit to 300_000
        let update_limit_msg = ExecuteMsg::UpdateWithdrawalLimit { limit: 300_000 };
        execute(deps.as_mut(), mock_env(), info.clone(), update_limit_msg).unwrap();

        // Attempt to withdraw 400_000, exceeding limit
        let withdraw_msg = ExecuteMsg::Withdraw { amount: 400_000 };
        let err = execute(deps.as_mut(), mock_env(), info.clone(), withdraw_msg).unwrap_err();

        assert_eq!(err, ContractError::WithdrawalLimitExceeded {});
        assert_eq!(TOTAL_SUPPLY.load(&deps.storage).unwrap(), 1_000_000); // Supply remains unchanged
    }
}
