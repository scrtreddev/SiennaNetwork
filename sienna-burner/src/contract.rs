use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr,
    InitResponse, Querier, StdError, StdResult, Storage, Uint128, log
};

use sienna_amm_shared::msg::sienna_burner::{HandleMsg, InitMsg, QueryAnswer, QueryMsg, ResponseStatus};
use sienna_amm_shared::ContractInfo;
use sienna_amm_shared::snip20;

use crate::state::*;

const BLOCK_SIZE: usize = 256;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    save_token_info(deps, &msg.sienna_token)?;
    save_burn_pool(deps, &msg.burn_pool)?;
    
    let admins = if let Some(mut admins) = msg.admins {
        admins.push(msg.factory_address);
        admins
    } else {
        vec![ msg.factory_address, env.message.sender ]
    };

    save_admins(deps, &admins)?;

    if let Some(pairs) = msg.pairs {
        save_pair_addresses(deps, &pairs)?;
    }

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Burn {
            amount
        } => burn(deps, env, amount),
        HandleMsg::AddPairs { pairs } => add_pairs(deps, env, pairs),
        HandleMsg::RemovePairs { pairs } => remove_pairs(deps, env, pairs),
        HandleMsg::AddAdmins { addresses } => add_admins(deps, env, addresses),
        HandleMsg::SetBurnPool {address } => set_burn_pool(deps, env, address),
        HandleMsg::SetSiennaToken { info } => set_token(deps, env, info)
    }
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::BurnPool => query_burn_pool(deps),
        QueryMsg::Admins => query_admins(deps),
        QueryMsg::SiennaToken => query_token(deps)
    }
}

fn burn<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    amount: Uint128
) -> StdResult<HandleResponse> {
    if !pair_address_exists(&deps, &env.message.sender)? {
        return Err(StdError::unauthorized());
    }

    let burn_pool = load_burn_pool(&deps)?;
    let sienna_token = load_token_info(&deps)?;

    Ok(HandleResponse {
        messages: vec![
            snip20::burn_from_msg(
                burn_pool,
                amount,
                None,
                BLOCK_SIZE,
                sienna_token.code_hash,
                sienna_token.address
            )?
        ],
        log: vec![
            log("sienna_burned", amount)
        ],
        data: Some(to_binary(&ResponseStatus::Success)?),
    })
}

fn add_pairs<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    pairs: Vec<HumanAddr>,
) -> StdResult<HandleResponse> {
    enforce_admin(deps, env)?;
    save_pair_addresses(deps, &pairs)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&ResponseStatus::Success)?),
    })
}

fn remove_pairs<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    pairs: Vec<HumanAddr>,
) -> StdResult<HandleResponse> {
    enforce_admin(deps, env)?;
    remove_pair_addresses(deps, &pairs)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&ResponseStatus::Success)?),
    })
}

fn add_admins<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    addresses: Vec<HumanAddr>,
) -> StdResult<HandleResponse> {
    enforce_admin(deps, env)?;
    save_admins(deps, &addresses)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&ResponseStatus::Success)?),
    })
}

fn set_burn_pool<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    address: HumanAddr,
) -> StdResult<HandleResponse> {
    enforce_admin(deps, env)?;
    save_burn_pool(deps, &address)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&ResponseStatus::Success)?),
    })
}

fn set_token<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    info: ContractInfo,
) -> StdResult<HandleResponse> {
    enforce_admin(deps, env)?;
    save_token_info(deps, &info)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&ResponseStatus::Success)?),
    })
}

fn enforce_admin<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<()> {
    let admins = load_admins(deps)?;

    if admins.contains(&env.message.sender) {
        return Ok(());
    }

    Err(StdError::unauthorized())
}

fn query_burn_pool<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<Binary> {
    let address = load_burn_pool(deps)?;

    to_binary(&QueryAnswer::BurnPool {
        address,
    })
}

fn query_admins<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<Binary> {
    let addresses = load_admins(deps)?;

    to_binary(&QueryAnswer::Admins { 
        addresses
    })
}

fn query_token<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<Binary> {
    let info = load_token_info(deps)?;

    to_binary(&QueryAnswer::SiennaToken { 
        info 
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::from_binary;
    use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, mock_dependencies, mock_env};

    const FACTORY: &'static str = "factory_address";
    const BURN_POOL: &'static str = "burn_pool";
    const SENDER: &'static str = "sender";

    fn initialize(
        pairs: Option<Vec<HumanAddr>>,
        admins: Option<Vec<HumanAddr>>
    ) -> StdResult<Extern<MockStorage, MockApi, MockQuerier>> {
        let env = mock_env(SENDER, &[]);
        let mut deps = mock_dependencies(20, &[]);

        let msg = InitMsg {
            sienna_token: ContractInfo {
                address: "sienna_token".into(),
                code_hash: "sienna_token_hash".into()
            },
            pairs,
            burn_pool: HumanAddr(BURN_POOL.into()),
            factory_address: HumanAddr(FACTORY.into()),
            admins
        };

        init(&mut deps, env.clone(), msg)?;

        Ok(deps)
    }

    fn assert_unauthorized(response: StdResult<HandleResponse>) {
        assert!(response.is_err());

        let err = response.unwrap_err();
        assert_eq!(err, StdError::unauthorized())
    }

    #[test]
    fn test_proper_init() -> StdResult<()> {
        let sienna_token = ContractInfo {
            address: "sienna_token".into(),
            code_hash: "sienna_token_hash".into()
        };

        let pairs = vec![ 
            HumanAddr("pair1".into()),
            HumanAddr("pair2".into()),
            HumanAddr("pair3".into())
        ];

        let ref mut deps = initialize(Some(pairs.clone()), None)?;

        assert_eq!(sienna_token, load_token_info(deps)?);
        assert_eq!(HumanAddr(BURN_POOL.into()), load_burn_pool(deps)?);
        assert_eq!(
            vec![ HumanAddr(FACTORY.into()), SENDER.into() ],
            load_admins(deps)?
        );

        for i in 0..pairs.len() {
            assert!(pair_address_exists(deps, &pairs[i])?);
        }

        let admins = vec![
            HumanAddr("admin1".into()),
            HumanAddr("admin2".into()),
            HumanAddr("admin3".into())
        ];
        
        let ref mut deps = initialize(None, Some(admins.clone()))?;

        assert_eq!(
            [ admins, vec![ HumanAddr(FACTORY.into()) ] ].concat(),
            load_admins(deps)?
        );

        Ok(())
    }

    #[test]
    fn test_burn() -> StdResult<()> {
        let pair_1 = "pair1";

        let pairs = vec![ 
            HumanAddr(pair_1.into()),
            HumanAddr("pair2".into()),
            HumanAddr("pair3".into())
        ];

        let ref mut deps = initialize(Some(pairs.clone()), None)?;

        let result = handle(
            deps,
            mock_env(pair_1, &[]),
            HandleMsg::Burn {
                amount: Uint128(100)
            }
        );

        assert!(result.is_ok());

        let result = handle(
            deps,
            mock_env("unauthorized", &[]),
            HandleMsg::Burn {
                amount: Uint128(100)
            }
        );

        assert_unauthorized(result);

        let new_pair = HumanAddr("new_pair".into());

        handle(
            deps,
            mock_env(SENDER, &[]),
            HandleMsg::AddPairs {
                pairs: vec![ new_pair.clone() ]
            }
        )?;

        let result = handle(
            deps,
            mock_env(new_pair, &[]),
            HandleMsg::Burn {
                amount: Uint128(100)
            }
        );

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn test_add_pairs() -> StdResult<()> {
        let pairs = vec![ 
            HumanAddr("pair1".into()),
            HumanAddr("pair2".into()),
            HumanAddr("pair3".into())
        ];

        let ref mut deps = initialize(Some(pairs.clone()), None)?;

        let new_pairs = vec![ 
            HumanAddr("pair4".into()),
            HumanAddr("pair5".into())
        ];

        handle(
            deps,
            mock_env(SENDER, &[]),
            HandleMsg::AddPairs {
                pairs: new_pairs.clone()
            }
        )?;

        let all_pairs = [ pairs, new_pairs ].concat();

        for i in 0..all_pairs.len() {
            assert!(pair_address_exists(deps, &all_pairs[i])?);
        }

        let result = handle(
            deps,
            mock_env("non_admin", &[]),
            HandleMsg::AddPairs {
                pairs: vec![ HumanAddr("whatever".into()) ]
            }
        );
        
        assert_unauthorized(result);

        Ok(())
    }

    #[test]
    fn test_remove_pairs() -> StdResult<()> {
        let pairs = vec![ 
            HumanAddr("pair1".into()),
            HumanAddr("pair2".into()),
            HumanAddr("pair3".into())
        ];

        let ref mut deps = initialize(Some(pairs.clone()), None)?;

        let remove_pairs = vec![ 
            HumanAddr("pair3".into()),
            HumanAddr("non_existant".into())
        ];

        let result = handle(
            deps,
            mock_env("non_admin", &[]),
            HandleMsg::RemovePairs {
                pairs: remove_pairs.clone()
            }
        );

        assert_unauthorized(result);

        handle(
            deps,
            mock_env(SENDER, &[]),
            HandleMsg::RemovePairs {
                pairs: remove_pairs.clone()
            }
        )?;

        assert!(pair_address_exists(deps, &pairs[0])?);
        assert!(pair_address_exists(deps, &pairs[1])?);
        assert!(!pair_address_exists(deps, &pairs[2])?);
        assert!(!pair_address_exists(deps, &remove_pairs[1])?);

        Ok(())
    }

    #[test]
    fn test_add_admins() -> StdResult<()> {
        let ref mut deps = initialize(None, None)?;

        let new_admins = vec![
            HumanAddr("new_admin1".into()),
            HumanAddr("new_admin2".into())
        ];

        let result = handle(
            deps,
            mock_env("non_admin", &[]),
            HandleMsg::AddAdmins {
                addresses: new_admins.clone()
            }
        );

        assert_unauthorized(result);

        handle(
            deps,
            mock_env(SENDER, &[]),
            HandleMsg::AddAdmins {
                addresses: new_admins.clone()
            }
        )?;

        let result = query(deps, QueryMsg::Admins)?;
        let result: QueryAnswer = from_binary(&result)?;

        match result {
            QueryAnswer::Admins { addresses } => {
                assert_eq!(
                    addresses,
                    [ 
                        vec![ HumanAddr(FACTORY.into()), HumanAddr(SENDER.into()) ],
                        new_admins.clone()
                    ].concat()
                )
            },
            _ => panic!("Expected QueryAnswer::Admins")
        }

        handle(
            deps,
            mock_env(new_admins[0].clone(), &[]),
            HandleMsg::AddAdmins {
                addresses: new_admins.clone()
            }
        )?;

        let result = query(deps, QueryMsg::Admins)?;
        let result: QueryAnswer = from_binary(&result)?;

        match result {
            QueryAnswer::Admins { addresses } => {
                assert!(addresses.len() == new_admins.len() * 2 + 2)
            },
            _ => panic!("Expected QueryAnswer::Admins")
        }

        Ok(())
    }

    #[test]
    fn test_set_burn_pool() -> StdResult<()> {
        let ref mut deps = initialize(None, None)?;
        
        let pool = HumanAddr("pool".into());

        let result = handle(
            deps,
            mock_env("non_admin", &[]),
            HandleMsg::SetBurnPool {
                address: pool.clone()
            }
        );

        assert_unauthorized(result);

        handle(
            deps,
            mock_env(SENDER, &[]),
            HandleMsg::SetBurnPool {
                address: pool.clone()
            }
        )?;

        let result = query(deps, QueryMsg::BurnPool)?;
        let result: QueryAnswer = from_binary(&result)?;

        match result {
            QueryAnswer::BurnPool { address } => {
                assert_eq!(pool, address)
            },
            _ => panic!("Expected QueryAnswer::BurnPool")
        }

        Ok(())
    }

    #[test]
    fn test_set_sienna_token() -> StdResult<()> {
        let ref mut deps = initialize(None, None)?;
        
        let token = ContractInfo {
            address: "new_token".into(),
            code_hash: "new_token_hash".into()
        };

        let result = handle(
            deps,
            mock_env("non_admin", &[]),
            HandleMsg::SetSiennaToken {
                info: token.clone()
            }
        );

        assert_unauthorized(result);

        handle(
            deps,
            mock_env(SENDER, &[]),
            HandleMsg::SetSiennaToken {
                info: token.clone()
            }
        )?;

        let result = query(deps, QueryMsg::SiennaToken)?;
        let result: QueryAnswer = from_binary(&result)?;

        match result {
            QueryAnswer::SiennaToken { info } => {
                assert_eq!(token, info)
            },
            _ => panic!("Expected QueryAnswer::SiennaToken")
        }

        Ok(())
    }
}
