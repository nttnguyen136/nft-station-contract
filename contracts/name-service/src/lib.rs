pub mod error;
pub mod execute;
pub mod msg;
pub mod query;
pub mod state;
pub mod utils;

use msg::Extension;

use cosmwasm_std::Empty;

pub type Cw721MetadataContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty>;

pub mod entry {
    use crate::error::ContractError;
    use crate::execute::{execute_instantiate, mint, update_metadata};
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
    use crate::query::address_of;

    use super::*;

    use cosmwasm_std::{entry_point, to_binary};
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    // This is a simple type to let us handle empty extensions

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        let contract = Cw721MetadataContract::default();

        execute_instantiate(contract, deps, env, info, msg)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        let contract = Cw721MetadataContract::default();

        match msg {
            ExecuteMsg::Mint(msg) => mint(contract, deps, env, info, msg),
            ExecuteMsg::UpdateMetadata(msg) => update_metadata(contract, deps, env, info, msg),
            _ => contract
                .execute(deps, env, info, msg.into())
                .map_err(ContractError::Base),
        }
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        let contract = Cw721MetadataContract::default(); // .query(deps, env, msg)

        match msg {
            QueryMsg::AddressOf { token_id } => to_binary(&address_of(contract, deps, token_id)?),
            _ => contract.query(deps, env, msg.into()).map_err(|err| err),
        }
    }
}
