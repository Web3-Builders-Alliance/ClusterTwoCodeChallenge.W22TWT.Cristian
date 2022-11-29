#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult,
    SubMsg,
};
// use cw2::set_contract_version;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

/*
const CONTRACT_NAME: &str = "crates.io:contract-a";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

const REDIRECT_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State{
        receiver: deps.api.addr_validate(&msg.receiver)?, //we validate our receiver (in this challenge we'll be contract B)
    };

    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("receiver", state.receiver.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RedirectFunds{} => redirect_funds(deps,info),
    }
}


pub fn redirect_funds(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let receiver = state.receiver;
    let sender = deps.api.addr_validate(&info.sender.to_string())?;

    let redirect_msg = BankMsg::Send {
        to_address: receiver.to_string(),
        amount: info.funds,
    };
    let redirect_submsg = SubMsg::reply_on_error(redirect_msg, REDIRECT_REPLY_ID);

    Ok(Response::new()
        .add_submessage(redirect_submsg)
        .add_attribute("method", "redirect funds")
        .add_attribute("from", sender.to_string())
        .add_attribute("to", receiver.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        REDIRECT_REPLY_ID => handle_redirect_reply(deps, msg), 
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))), // if we didn't match one of our reply id
    }
}


pub fn handle_redirect_reply(_deps: DepsMut, msg: Reply) -> StdResult<Response>{
    let _data = msg.result.into_result().map_err(StdError::generic_err);

    // Here we can do what we want with the error 
    
    // I'm just going to return a response that says handle redirect reply
    Ok(Response::new().add_attribute("method", "handle_redirect_reply"))

}

#[cfg(test)]
mod tests {
    use super::instantiate;
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::{
        attr,
        testing::{mock_dependencies, mock_env, mock_info},
    };

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("juno1k48p45n509dgpmq4akf3079fzt4yquwz5swl3s", &[]);
        let msg = InstantiateMsg {
            receiver: "juno1gqhgn5825qg38up8saaduxzsyrdwgur2c66y40".to_string(),
        };

        let response = instantiate(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(response.attributes, vec![attr("method", "instantiate")]);
    }
}
