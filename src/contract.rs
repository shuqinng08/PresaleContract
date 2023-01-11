use cosmwasm_std::{
    entry_point, to_binary, BankMsg, Coin, CosmosMsg, Decimal, Deps, DepsMut, Env, MessageInfo,
    Response, StdResult, Uint128, WasmMsg,
};

use cw2::set_contract_version;
use cw20::Cw20ExecuteMsg;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{user_info_key, user_info_storage, SaleInfo, State, UserInfo, CONFIG, SALEINFO};

const CONTRACT_NAME: &str = "Hope_Presale";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const JUNO: &str = "ujuno";
const ATOM: &str = "ibc/C4CFF46FD6DE35CA4CF4CE031E643C8FDC9BA4B99AE598E9B0ED98FE3A2319F9";
const USDC: &str = "ibc/EAC38D55372F38F1AFD68DF7FE9EF762DCF69F26520643CF3F9D292A738D8034";

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    deps.api.addr_validate(&msg.admin)?;
    deps.api.addr_validate(&msg.token_address)?;

    let crr_time = env.block.time.seconds();
    if crr_time > msg.presale_start || (msg.presale_start + msg.presale_period) > msg.claim_start {
        return Err(ContractError::WrongConfig {});
    }

    //presale start, end and claim period check
    let state = State {
        admin: msg.admin,
        token_address: msg.token_address,
        total_supply: msg.total_supply,
        presale_start: msg.presale_start,
        presale_period: msg.presale_period,
        vesting_step_period: msg.vesting_step_period,
        claim_start: msg.claim_start,
        token_cost_atom: msg.token_cost_atom,
        token_cost_juno: msg.token_cost_juno,
        token_cost_usdc: msg.token_cost_usdc,
        contract_admin: info.sender.to_string(),
    };
    CONFIG.save(deps.storage, &state)?;

    SALEINFO.save(
        deps.storage,
        &SaleInfo {
            token_sold_amount: Uint128::zero(),
            earned_atom: Uint128::zero(),
            earned_juno: Uint128::zero(),
            earned_usdc: Uint128::zero(),
        },
    )?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::BuyToken {} => execute_buy_token(deps, env, info),
        ExecuteMsg::ChangeAdmin { address } => execute_change_admin(deps, env, info, address),
        ExecuteMsg::ClaimToken {} => execute_claim_token(deps, env, info),
        ExecuteMsg::UpdateConfig { state } => execute_update_config(deps, env, info, state),
        ExecuteMsg::WithdrawTokenByAdmin {} => execute_withdraw_token_by_admin(deps, env, info),
    }
}

fn execute_buy_token(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let state = CONFIG.load(deps.storage)?;
    let sender = info.sender.to_string();

    //presale start validation check
    let crr_time = env.block.time.seconds();
    if crr_time < state.presale_start {
        return Err(ContractError::PresaleNotStarted {});
    };

    if crr_time > state.presale_start + state.presale_period {
        return Err(ContractError::PresaleEnded {});
    }

    let received_coin = get_coin_info(&info)?;
    let token_amount: Uint128;
    let sale_info = SALEINFO.load(deps.storage)?;

    let message: CosmosMsg;

    let user_info_key = user_info_key(&sender);

    if received_coin.denom.as_str() == ATOM {
        token_amount = received_coin.amount * state.token_cost_atom;

        if token_amount + sale_info.token_sold_amount > state.total_supply {
            return Err(ContractError::NoEnoughTokens {});
        }
        //sale info update
        SALEINFO.update(deps.storage, |mut sale_info| -> StdResult<_> {
            sale_info.earned_atom = sale_info.earned_atom + received_coin.amount;
            sale_info.token_sold_amount = sale_info.token_sold_amount + token_amount;
            Ok(sale_info)
        })?;

        //user info update
        let user_info = user_info_storage().may_load(deps.storage, user_info_key.clone())?;
        match user_info {
            Some(mut user_info) => {
                user_info.sent_atom = user_info.sent_atom + received_coin.amount;
                user_info.total_claim_amount = user_info.total_claim_amount + token_amount;
                user_info_storage().save(deps.storage, user_info_key, &user_info)?;
            }
            None => {
                user_info_storage().save(
                    deps.storage,
                    user_info_key,
                    &UserInfo {
                        address: sender.clone(),
                        total_claim_amount: token_amount,
                        sent_atom: received_coin.amount,
                        sent_juno: Uint128::zero(),
                        sent_usdc: Uint128::zero(),
                        claimed_amount: Uint128::zero(),
                        vesting_step: 0,
                        last_received: 0,
                    },
                )?;
            }
        }
        message = CosmosMsg::Bank(BankMsg::Send {
            to_address: state.admin,
            amount: vec![Coin {
                denom: received_coin.denom.clone(),
                amount: received_coin.amount,
            }],
        })
    } else if received_coin.denom.as_str() == JUNO {
        token_amount = received_coin.amount * state.token_cost_juno;

        if token_amount + sale_info.token_sold_amount > state.total_supply {
            return Err(ContractError::NoEnoughTokens {});
        }
        //sale info update
        SALEINFO.update(deps.storage, |mut sale_info| -> StdResult<_> {
            sale_info.earned_juno = sale_info.earned_juno + received_coin.amount;
            sale_info.token_sold_amount = sale_info.token_sold_amount + token_amount;
            Ok(sale_info)
        })?;

        //user info update
        let user_info = user_info_storage().may_load(deps.storage, user_info_key.clone())?;
        match user_info {
            Some(mut user_info) => {
                user_info.sent_juno = user_info.sent_juno + received_coin.amount;
                user_info.total_claim_amount = user_info.total_claim_amount + token_amount;
                user_info_storage().save(deps.storage, user_info_key, &user_info)?;
            }
            None => {
                user_info_storage().save(
                    deps.storage,
                    user_info_key,
                    &UserInfo {
                        address: sender.clone(),
                        total_claim_amount: token_amount,
                        sent_juno: received_coin.amount,
                        sent_atom: Uint128::zero(),
                        sent_usdc: Uint128::zero(),
                        claimed_amount: Uint128::zero(),
                        vesting_step: 0,
                        last_received: 0,
                    },
                )?;
            }
        }
        message = CosmosMsg::Bank(BankMsg::Send {
            to_address: state.admin,
            amount: vec![Coin {
                denom: received_coin.denom.clone(),
                amount: received_coin.amount,
            }],
        })
    } else {
        token_amount = received_coin.amount * state.token_cost_usdc;

        if token_amount + sale_info.token_sold_amount > state.total_supply {
            return Err(ContractError::NoEnoughTokens {});
        }
        //sale info update
        SALEINFO.update(deps.storage, |mut sale_info| -> StdResult<_> {
            sale_info.earned_usdc = sale_info.earned_usdc + received_coin.amount;
            sale_info.token_sold_amount = sale_info.token_sold_amount + token_amount;
            Ok(sale_info)
        })?;

        //user info update
        let user_info = user_info_storage().may_load(deps.storage, user_info_key.clone())?;
        match user_info {
            Some(mut user_info) => {
                user_info.sent_usdc = user_info.sent_usdc + received_coin.amount;
                user_info.total_claim_amount = user_info.total_claim_amount + token_amount;
                user_info_storage().save(deps.storage, user_info_key, &user_info)?;
            }
            None => {
                user_info_storage().save(
                    deps.storage,
                    user_info_key,
                    &UserInfo {
                        address: sender.clone(),
                        total_claim_amount: token_amount,
                        sent_usdc: received_coin.amount,
                        sent_juno: Uint128::zero(),
                        sent_atom: Uint128::zero(),
                        claimed_amount: Uint128::zero(),
                        vesting_step: 0,
                        last_received: 0,
                    },
                )?;
            }
        }
        message = CosmosMsg::Bank(BankMsg::Send {
            to_address: state.admin,
            amount: vec![Coin {
                denom: received_coin.denom.clone(),
                amount: received_coin.amount,
            }],
        })
    }

    if sale_info.token_sold_amount + token_amount == state.total_supply {
        CONFIG.update(deps.storage, |mut state| -> StdResult<_> {
            state.presale_period = crr_time - state.presale_start;
            state.claim_start = crr_time + state.vesting_step_period;
            Ok(state)
        })?;
    }

    Ok(Response::new()
        .add_attribute("action", "buy_token")
        .add_attribute("denom", received_coin.denom)
        .add_attribute("amount", received_coin.amount.to_string())
        .add_attribute("buyer", sender)
        .add_message(message))
}

fn execute_claim_token(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let sender = info.sender.to_string();
    let state = CONFIG.load(deps.storage)?;
    let crr_time = env.block.time.seconds();
    let presale_end = state.presale_start + state.presale_period;

    if crr_time < presale_end {
        return Err(ContractError::PresaleNotEnded {});
    }

    let mut messages: Vec<CosmosMsg> = Vec::new();
    let user_info_key = user_info_key(&sender);
    let user_info = user_info_storage().may_load(deps.storage, user_info_key)?;

    let first_portion = Decimal::from_ratio(1 as u128, 10 as u128);
    let default_portion = Decimal::from_ratio(15 as u128, 100 as u128);

    if crr_time < state.claim_start {
        match user_info {
            Some(user_info) => {
                if user_info.vesting_step == 0 {
                    let token_amount_to_send = first_portion * user_info.total_claim_amount;

                    user_info_update(
                        deps,
                        sender.clone(),
                        token_amount_to_send,
                        crr_time,
                        1,
                        state,
                        &mut messages,
                    )?;
                } else {
                    return Err(ContractError::AlreadyClaimedForCurrentStep {});
                }
            }
            None => return Err(ContractError::NotInPresale {}),
        }
    } else {
        match user_info {
            Some(user_info) => {
                let mut expect_step =
                    (crr_time - state.claim_start) / state.vesting_step_period + 2;
                if expect_step > 7 {
                    expect_step = 7;
                }
                if user_info.vesting_step == expect_step {
                    return Err(ContractError::AlreadyClaimedForCurrentStep {});
                } else {
                    if user_info.vesting_step == 0 {
                        let token_amount_to_send = first_portion * user_info.total_claim_amount
                            + Uint128::from((expect_step - 1) as u128)
                                * user_info.total_claim_amount
                                * default_portion;
                        user_info_update(
                            deps,
                            sender.clone(),
                            token_amount_to_send,
                            crr_time,
                            expect_step,
                            state,
                            &mut messages,
                        )?;
                    } else {
                        println!("step{:?},{:?}", expect_step, user_info.vesting_step);
                        let token_amount_to_send =
                            Uint128::from((expect_step - user_info.vesting_step) as u128)
                                * user_info.total_claim_amount
                                * default_portion;
                        user_info_update(
                            deps,
                            sender.clone(),
                            token_amount_to_send,
                            crr_time,
                            expect_step,
                            state,
                            &mut messages,
                        )?;
                    }
                }
            }
            None => return Err(ContractError::NotInPresale {}),
        }
    }

    if messages.is_empty() {
        Ok(Response::new()
            .add_attribute("action", "claim token")
            .add_attribute("user", sender))
    } else {
        Ok(Response::new()
            .add_attribute("action", "claim token")
            .add_attribute("user", sender)
            .add_messages(messages))
    }
}

//Mint token to this contract
fn execute_change_admin(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    authcheck(deps.as_ref(), &info)?;

    CONFIG.update(deps.storage, |mut state| -> StdResult<_> {
        state.admin = address.clone();
        Ok(state)
    })?;

    Ok(Response::new()
        .add_attribute("action", "change the admin")
        .add_attribute("address", address))
}

fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    state: State,
) -> Result<Response, ContractError> {
    authcheck(deps.as_ref(), &info)?;

    CONFIG.save(deps.storage, &state)?;
    Ok(Response::new().add_attribute("action", "update configuration"))
}

fn execute_withdraw_token_by_admin(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    authcheck(deps.as_ref(), &info)?;

    let state = CONFIG.load(deps.storage)?;
    let sale_info = SALEINFO.load(deps.storage)?;
    let crr_time = env.block.time.seconds();
    let presale_end = state.presale_start + state.presale_period;

    if crr_time < presale_end {
        return Err(ContractError::PresaleNotEnded {});
    }

    let cw20_transfer_msg = WasmMsg::Execute {
        contract_addr: "token_address".to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: state.admin,
            amount: state.total_supply - sale_info.token_sold_amount,
        })?,
        funds: vec![],
    };

    let msg: CosmosMsg = CosmosMsg::Wasm(cw20_transfer_msg);

    Ok(Response::new()
        .add_attribute("action", "withdraw token by admin")
        .add_message(msg))
}

fn authcheck(deps: Deps, info: &MessageInfo) -> Result<(), ContractError> {
    let state = CONFIG.load(deps.storage)?;
    if info.sender.to_string() != state.contract_admin {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

fn get_coin_info(info: &MessageInfo) -> Result<Coin, ContractError> {
    if info.funds.len() != 1 {
        return Err(ContractError::SeveralCoinsSent {});
    } else {
        let denom = info.funds[0].denom.clone();
        if denom.as_str() != ATOM && denom.as_str() != JUNO && denom.as_str() != USDC {
            return Err(ContractError::NoExistCoin {});
        }
        Ok(info.funds[0].clone())
    }
}

fn user_info_update(
    deps: DepsMut,
    sender: String,
    token_amount_to_send: Uint128,
    crr_time: u64,
    expect_step: u64,
    state: State,
    messages: &mut Vec<CosmosMsg>,
) -> StdResult<()> {
    let user_info_key = user_info_key(&sender);
    user_info_storage().update(deps.storage, user_info_key, |user_info| -> StdResult<_> {
        let mut user_info = user_info.unwrap();
        user_info.vesting_step = expect_step;
        user_info.last_received = crr_time;
        user_info.claimed_amount = user_info.claimed_amount + token_amount_to_send;
        Ok(user_info)
    })?;

    let transfer_msg = WasmMsg::Execute {
        contract_addr: state.token_address,
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: sender.clone(),
            amount: token_amount_to_send,
        })?,
        funds: vec![],
    };

    messages.push(CosmosMsg::Wasm(transfer_msg));

    Ok(())
}
