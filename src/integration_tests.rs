#[cfg(test)]
mod integration_tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_binary, Addr};

    use crate::contract::{instantiate, execute, query};
    use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
    use crate::state::{OWNER, TOTAL_SUPPLY, PEG_PRICE};

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
    fn test_mint() {
        let mut deps = mock_dependencies();

        let init_msg = InstantiateMsg {
            initial_supply: 1_000_000,
            peg_price: 1_000,
        };
        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg).unwrap();

        let exec_msg = ExecuteMsg::Mint { amount: 500_000 };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), exec_msg).unwrap();

        assert_eq!(res.messages.len(), 0);
        assert_eq!(TOTAL_SUPPLY.load(&deps.storage).unwrap(), 1_500_000);
    }

    #[test]
    fn test_burn() {
        let mut deps = mock_dependencies();

        let init_msg = InstantiateMsg {
            initial_supply: 1_000_000,
            peg_price: 1_000,
        };
        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg).unwrap();

        let exec_msg = ExecuteMsg::Burn { amount: 400_000 };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), exec_msg).unwrap();

        assert_eq!(res.messages.len(), 0);
        assert_eq!(TOTAL_SUPPLY.load(&deps.storage).unwrap(), 600_000);
    }

    #[test]
    fn test_update_peg_price() {
        let mut deps = mock_dependencies();

        let init_msg = InstantiateMsg {
            initial_supply: 1_000_000,
            peg_price: 1_000,
        };
        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg).unwrap();

        let exec_msg = ExecuteMsg::UpdatePegPrice { peg_price: 1_500 };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), exec_msg).unwrap();

        assert_eq!(res.messages.len(), 0);
        assert_eq!(PEG_PRICE.load(&deps.storage).unwrap(), 1_500);
    }
}
