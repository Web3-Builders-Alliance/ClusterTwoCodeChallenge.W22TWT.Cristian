#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Coin, SubMsg, BankMsg, to_binary};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

/*
const CONTRACT_NAME: &str = "crates.io:contract-b";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
 */

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State{
        admin: deps.api.addr_validate(&msg.admin.to_string())?
    };

    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", state.admin.to_string()))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Withdraw{address} => withdraw_funds(deps, info, address),
    };
    unimplemented!()
}

pub fn withdraw_funds(deps: DepsMut, info: MessageInfo, address: String) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.admin {  // only the admin can withdraw funds from this contract
        return Err(ContractError::Unauthorazied {});
    }

    let verified_addr = deps.api.addr_validate(&address)?; // the address where we want to whitdraw the funds
    let _whitdraw_msg: SubMsg = SubMsg::new(BankMsg::Send{to_address: verified_addr.to_string(), amount: info.funds});



    Ok(Response::new()
        .add_attribute("method", "whitdraw")
        .add_attribute("to", verified_addr.to_string()))
    


}

// i have a problem with:           use cosmwasm_schema::{QueryResponse,cw_serde};      in msg.rs file.
/*#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAdmin {} => to_binary(&get_admin(deps)?),
    }
}

pub fn get_admin(deps: Deps) -> StdResult<GetAdminResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetAdminResponse { admin: state.admin })
}*/

#[cfg(test)]
mod tests {}
