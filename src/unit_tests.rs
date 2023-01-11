#[cfg(test)]
use crate::contract::{execute, instantiate};
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::query::{
    query_claimable_amount, query_claimable_time, query_get_user_infos, query_sale_info,
    query_state_info, query_user_info,
};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{to_binary, BankMsg, Coin, CosmosMsg, DepsMut, Env, Uint128, WasmMsg};

use cw20::Cw20ExecuteMsg;

const ATOM: &str = "ibc/C4CFF46FD6DE35CA4CF4CE031E643C8FDC9BA4B99AE598E9B0ED98FE3A2319F9";
const USDC: &str = "ibc/EAC38D55372F38F1AFD68DF7FE9EF762DCF69F26520643CF3F9D292A738D8034";

fn setup_contract(deps: DepsMut, env: Env) {
    let instantiate_msg = InstantiateMsg {
        admin: "admin".to_string(),
        token_address: "token_address".to_string(),
        total_supply: Uint128::new(10000),
        presale_start: env.block.time.seconds() + 300,
        presale_period: 100,
        vesting_step_period: 200,
        claim_start: env.block.time.seconds() + 500,
        token_cost_juno: Uint128::new(50),
        token_cost_atom: Uint128::new(100),
        token_cost_usdc: Uint128::new(10),
    };
    let info = mock_info("owner", &[]);
    let res = instantiate(deps, mock_env(), info, instantiate_msg).unwrap();
    assert_eq!(res.messages.len(), 0);
}

#[test]
fn init_contract() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let instantiate_msg = InstantiateMsg {
        admin: "admin".to_string(),
        token_address: "token_address".to_string(),
        total_supply: Uint128::new(100000),
        presale_start: env.block.time.seconds() + 300,
        presale_period: 100,
        vesting_step_period: 200,
        claim_start: env.block.time.seconds() + 500,
        token_cost_juno: Uint128::new(50),
        token_cost_atom: Uint128::new(100),
        token_cost_usdc: Uint128::new(10),
    };
    let info = mock_info("owner", &[]);
    let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
    assert_eq!(0, res.messages.len());
}

#[test]
fn test_buy() {
    let mut deps = mock_dependencies();
    let mut env = mock_env();
    setup_contract(deps.as_mut(), env.clone());

    env.block.time = env.block.time.plus_seconds(300);

    let state_info = query_state_info(deps.as_ref()).unwrap();
    println!("{:?}", state_info);

    env.block.time = env.block.time.plus_seconds(20);

    let info = mock_info(
        "user1",
        &[Coin {
            denom: "ujuno".to_string(),
            amount: Uint128::new(200),
        }],
    );
    let msg = ExecuteMsg::BuyToken {};
    execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    let state_info = query_state_info(deps.as_ref()).unwrap();
    println!("{:?}", state_info);

    // let info = mock_info(
    //     "user1",
    //     &[Coin {
    //         denom: "ujuno".to_string(),
    //         amount: Uint128::new(15),
    //     }],
    // );
    // let msg = ExecuteMsg::BuyToken {};
    // let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    // assert_eq!(res.messages.len(), 1);
    // assert_eq!(
    //     res.messages[0].msg,
    //     CosmosMsg::Bank(BankMsg::Send {
    //         to_address: "admin".to_string(),
    //         amount: vec![Coin {
    //             denom: "ujuno".to_string(),
    //             amount: Uint128::new(15)
    //         }]
    //     })
    // );

    // let info = mock_info(
    //     "user2",
    //     &[Coin {
    //         denom: "ujuno".to_string(),
    //         amount: Uint128::new(20),
    //     }],
    // );
    // let msg = ExecuteMsg::BuyToken {};
    // execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    // let info = mock_info(
    //     "user1",
    //     &[Coin {
    //         denom: ATOM.to_string(),
    //         amount: Uint128::new(10),
    //     }],
    // );
    // let msg = ExecuteMsg::BuyToken {};
    // execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    // let info = mock_info(
    //     "user1",
    //     &[Coin {
    //         denom: ATOM.to_string(),
    //         amount: Uint128::new(10),
    //     }],
    // );
    // let msg = ExecuteMsg::BuyToken {};
    // execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    // let info = mock_info(
    //     "user2",
    //     &[Coin {
    //         denom: ATOM.to_string(),
    //         amount: Uint128::new(20),
    //     }],
    // );
    // let msg = ExecuteMsg::BuyToken {};
    // let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    // assert_eq!(res.messages.len(), 1);
    // assert_eq!(
    //     res.messages[0].msg,
    //     CosmosMsg::Bank(BankMsg::Send {
    //         to_address: "admin".to_string(),
    //         amount: vec![Coin {
    //             denom: ATOM.to_string(),
    //             amount: Uint128::new(20)
    //         }]
    //     })
    // );
    // let info = mock_info(
    //     "user2",
    //     &[Coin {
    //         denom: USDC.to_string(),
    //         amount: Uint128::new(20),
    //     }],
    // );
    // let msg = ExecuteMsg::BuyToken {};
    // execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    // let info = mock_info(
    //     "user2",
    //     &[Coin {
    //         denom: USDC.to_string(),
    //         amount: Uint128::new(20),
    //     }],
    // );
    // let msg = ExecuteMsg::BuyToken {};
    // execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    // let sale_info = query_sale_info(deps.as_ref()).unwrap();
    // println!("sale_info : {:?}", sale_info);

    // let user_info = query_user_info(deps.as_ref(), "user1".to_string()).unwrap();
    // println!("user1 information {:?}", user_info);

    // let user_info = query_user_info(deps.as_ref(), "user2".to_string()).unwrap();
    // println!("user2 information {:?}", user_info);

    // env.block.time = env.block.time.plus_seconds(300);

    // // let info = mock_info("admin", &[]);
    // // let msg = ExecuteMsg::WithdrawTokenByAdmin {};
    // // let res = execute(deps.as_mut(), env, info, msg).unwrap();

    // // assert_eq!(res.messages.len(), 1);
    // // assert_eq!(
    // //     res.messages[0].msg,
    // //     CosmosMsg::Wasm(WasmMsg::Execute {
    // //         contract_addr: "token_address".to_string(),
    // //         msg: to_binary(&Cw20ExecuteMsg::Transfer {
    // //             recipient: "admin".to_string(),
    // //             amount: Uint128::new(3750)
    // //         })
    // //         .unwrap(),
    // //         funds: vec![]
    // //     })
    // // );

    // println!("user informations");

    // let user_infos =
    //     query_get_user_infos(deps.as_ref(), Some("user1".to_string()), Some(20)).unwrap();
    // println!("{:?}", user_infos);
}

#[test]
fn claim() {
    let mut deps = mock_dependencies();
    let mut env = mock_env();
    setup_contract(deps.as_mut(), env.clone());

    env.block.time = env.block.time.plus_seconds(300);

    let info = mock_info(
        "user1",
        &[Coin {
            denom: "ujuno".to_string(),
            amount: Uint128::new(10),
        }],
    );
    let msg = ExecuteMsg::BuyToken {};
    execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    let info = mock_info(
        "user2",
        &[Coin {
            denom: "ujuno".to_string(),
            amount: Uint128::new(10),
        }],
    );
    let msg = ExecuteMsg::BuyToken {};
    execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    env.block.time = env.block.time.plus_seconds(250);

    let claimable_amount =
        query_claimable_amount(deps.as_ref(), env.clone(), "user1".to_string()).unwrap();
    println!("{:?}", claimable_amount);

    let info = mock_info("user1", &[]);
    let msg = ExecuteMsg::ClaimToken {};
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    let claimable_time =
        query_claimable_time(deps.as_ref(), env.clone(), "user1".to_string()).unwrap();
    println!(
        "{:?}",
        claimable_time.claimable_time - claimable_time.crr_time
    );

    assert_eq!(res.messages.len(), 1);
    assert_eq!(
        res.messages[0].msg,
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: "token_address".to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: "user1".to_string(),
                amount: Uint128::new(125)
            })
            .unwrap(),
            funds: vec![]
        })
    );

    env.block.time = env.block.time.plus_seconds(150);

    let claimable_amount =
        query_claimable_amount(deps.as_ref(), env.clone(), "user1".to_string()).unwrap();
    println!("{:?}", claimable_amount);

    let claimable_time =
        query_claimable_time(deps.as_ref(), env.clone(), "user1".to_string()).unwrap();
    println!(
        "{:?}, {:?}",
        claimable_time.claimable_time, claimable_time.crr_time
    );

    let info = mock_info("user1", &[]);
    let msg = ExecuteMsg::ClaimToken {};
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    let claimable_time =
        query_claimable_time(deps.as_ref(), env.clone(), "user1".to_string()).unwrap();
    println!(
        "{:?}",
        claimable_time.claimable_time - claimable_time.crr_time
    );

    assert_eq!(res.messages.len(), 1);
    assert_eq!(
        res.messages[0].msg,
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: "token_address".to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: "user1".to_string(),
                amount: Uint128::new(75)
            })
            .unwrap(),
            funds: vec![]
        })
    );

    let user_info = query_user_info(deps.as_ref(), "user1".to_string()).unwrap();
    println!("{:?}", user_info);

    env.block.time = env.block.time.plus_seconds(400);

    let claimable_amount =
        query_claimable_amount(deps.as_ref(), env.clone(), "user1".to_string()).unwrap();
    println!("claimable amount{:?}", claimable_amount);

    let info = mock_info("user1", &[]);
    let msg = ExecuteMsg::ClaimToken {};
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    assert_eq!(res.messages.len(), 1);
    assert_eq!(
        res.messages[0].msg,
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: "token_address".to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: "user1".to_string(),
                amount: Uint128::new(150)
            })
            .unwrap(),
            funds: vec![]
        })
    );

    let user_info = query_user_info(deps.as_ref(), "user1".to_string()).unwrap();
    println!("{:?}", user_info);

    env.block.time = env.block.time.plus_seconds(2000);

    let claimable_amount =
        query_claimable_amount(deps.as_ref(), env.clone(), "user1".to_string()).unwrap();
    println!("claimable amount{:?}", claimable_amount);

    let info = mock_info("user1", &[]);
    let msg = ExecuteMsg::ClaimToken {};
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    assert_eq!(res.messages.len(), 1);
    assert_eq!(
        res.messages[0].msg,
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: "token_address".to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: "user1".to_string(),
                amount: Uint128::new(150)
            })
            .unwrap(),
            funds: vec![]
        })
    );

    let user_info = query_user_info(deps.as_ref(), "user1".to_string()).unwrap();
    println!("{:?}", user_info);

    let info = mock_info("user2", &[]);
    let msg = ExecuteMsg::ClaimToken {};
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    assert_eq!(res.messages.len(), 1);
    assert_eq!(
        res.messages[0].msg,
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: "token_address".to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: "user2".to_string(),
                amount: Uint128::new(500)
            })
            .unwrap(),
            funds: vec![]
        })
    );

    let user_info = query_user_info(deps.as_ref(), "user2".to_string()).unwrap();
    println!("{:?}", user_info);

    env.block.time = env.block.time.plus_seconds(2000);

    let claimable_amount =
        query_claimable_amount(deps.as_ref(), env.clone(), "user1".to_string()).unwrap();
    println!("claimable amount {:?}", claimable_amount);

    let claimable_time =
        query_claimable_time(deps.as_ref(), env.clone(), "user2".to_string()).unwrap();
    println!("claimable time {:?}", claimable_time);
}
