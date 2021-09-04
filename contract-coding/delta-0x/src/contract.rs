use cosmwasm_bignumber::{Decimal256, Uint256};
use cosmwasm_std::*;
// {
//     Decimal, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
//     StdResult, Storage, WasmMsg, QueryRequest, WasmQuery, HumanAddr, Coin, CosmosMsg, BankMsg, Uint128, from_binary
// }

use crate::msg::{CountResponse, HandleMsg, AnchorHandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State};
use moneymarket::querier::{deduct_tax, query_balance, query_supply};
use mirror_protocol::mint::{
    Cw20HookMsg, PositionResponse, PositionsResponse,
};
use mirror_protocol::mint::HandleMsg as MirrorHandleMsg;
use cw20::{Cw20HandleMsg};
use terraswap::asset::{Asset, AssetInfo};
use cosmwasm_storage::to_length_prefixed;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        count: msg.count,
        owner: deps.api.canonical_address(&env.message.sender)?,
    };

    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

pub fn try_deposit<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    // count: i32,
) -> StdResult<HandleResponse> {
    let state = config_read(&deps.storage).load()?;

    let deposit_amount: Uint128 = env
        .message
        .sent_funds
        .iter()
        .find(|c| c.denom == "uusd")
        .map(|c| Uint128::from(c.amount))
        .unwrap_or_else(Uint128::zero);

    // Cannot deposit zero amount
    if deposit_amount.is_zero() {
        return Err(StdError::generic_err(format!(
            "Deposit amount must be greater than 0 {}",
            "uusd",
        )));
    }

    // TODO: add a function to update anchor smart contract address
    // contract_addr from https://docs.anchorprotocol.com/smart-contracts/deployed-contracts
    let msg = AnchorHandleMsg::DepositStable {};
    let exec = WasmMsg::Execute {
        contract_addr: HumanAddr("terra15dwd5mj8v59wpj0wvt233mf5efdff808c5tkal".to_string()),
        msg: to_binary(&msg)?,
        send: vec![deduct_tax(&deps, Coin {
            denom: "uusd".to_string(),
            amount: deposit_amount ,
        })?],
    };

    Ok(HandleResponse {
        messages: vec![CosmosMsg::Wasm(exec)],
        data: None,
        log: vec![],
    })
}

// pub fn try_open_mirror_position<S: Storage, A: Api, Q: Querier>(
//     deps: &mut Extern<S, A, Q>,
//     env: Env,
//     amount: Uint128
// ) -> StdResult<HandleResponse> {
//     let state = config_read(&deps.storage).load()?;
//     Ok(HandleResponse::default())
// }

pub fn try_open_mirror_position<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    amount: Uint128
) -> StdResult<HandleResponse> {
    let state = config_read(&deps.storage).load()?;

    // send all of the user's aust funds in the contract to mirror
    let mirrorOpenPositionMsg = MirrorHandleMsg::OpenPosition {
        collateral: Asset {
            info: AssetInfo::Token {
                // aUST on testnet
                contract_addr: HumanAddr::from("terra1ajt556dpzvjwl0kl5tzku3fc3p3knkg9mkv8jl"),
            },
            amount: amount,
        },
        asset_info: AssetInfo::Token {
            // mEth on testnet
            contract_addr: HumanAddr::from("terra1ys4dwwzaenjg2gy02mslmc96f267xvpsjat7gx"),
        },
        collateral_ratio: Decimal::percent(250),
        short_params: None,
    };

    // source: https://finder.terra.money/tequila-0004/tx/987678DDEFAC273C4164D01612547EFA4E91A2A6451F87FA499BA73BCAF06EF1
    let msg = Cw20HandleMsg::Send {
        contract: HumanAddr("terra1s9ehcjv0dqj2gsl72xrpp0ga5fql7fj7y3kq3w".to_string()), // mirror contract msg
        msg: Some(to_binary(&mirrorOpenPositionMsg)?),
        amount: amount
    };

    // source: https://docs.mirror.finance/networks
    // source: https://finder.terra.money/tequila-0004/tx/987678DDEFAC273C4164D01612547EFA4E91A2A6451F87FA499BA73BCAF06EF1
    let exec = WasmMsg::Execute {
        contract_addr: HumanAddr("terra1ajt556dpzvjwl0kl5tzku3fc3p3knkg9mkv8jl".to_string()), // aust contract address
        msg: to_binary(&msg)?,
        send: vec![],
    };

    Ok(HandleResponse {
        messages: vec![CosmosMsg::Wasm(exec)],
        data: None,
        log: vec![],
    })
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Increment {} => try_increment(deps, env),
        HandleMsg::Reset { count } => try_reset(deps, env, count),
        HandleMsg::Deposit {} => try_deposit(deps, env),
        HandleMsg::OpenMirrorPosition { amount } => try_open_mirror_position(deps, env, amount)
    }
}

pub fn try_increment<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
) -> StdResult<HandleResponse> {
    config(&mut deps.storage).update(|mut state| {
        state.count += 1;
        Ok(state)
    })?;

    Ok(HandleResponse::default())
}

pub fn try_reset<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    count: i32,
) -> StdResult<HandleResponse> {
    let api = &deps.api;
    config(&mut deps.storage).update(|mut state| {
        if api.canonical_address(&env.message.sender)? != state.owner {
            return Err(StdError::unauthorized());
        }
        state.count = count;
        Ok(state)
    })?;
    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
    }
}

fn query_count<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<CountResponse> {
    let state = config_read(&deps.storage).load()?;
    Ok(CountResponse { count: state.count })
}

#[inline]
fn concat(namespace: &[u8], key: &[u8]) -> Vec<u8> {
    let mut k = namespace.to_vec();
    k.extend_from_slice(key);
    k
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary, StdError};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(20, &[]);

        let msg = InitMsg { count: 17 };
        let env = mock_env("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let msg = InitMsg { count: 17 };
        let env = mock_env("creator", &coins(2, "token"));
        let _res = init(&mut deps, env, msg).unwrap();

        // beneficiary can release it
        let env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::Increment {};
        let _res = handle(&mut deps, env, msg).unwrap();

        // should increase counter by 1
        let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let msg = InitMsg { count: 17 };
        let env = mock_env("creator", &coins(2, "token"));
        let _res = init(&mut deps, env, msg).unwrap();

        // beneficiary can release it
        let unauth_env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::Reset { count: 5 };
        let res = handle(&mut deps, unauth_env, msg);
        match res {
            Err(StdError::Unauthorized { .. }) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_env = mock_env("creator", &coins(2, "token"));
        let msg = HandleMsg::Reset { count: 5 };
        let _res = handle(&mut deps, auth_env, msg).unwrap();

        // should now be 5
        let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }
}
