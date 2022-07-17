use cosmwasm_std::{Deps, StdError, StdResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Cw721MetadataContract;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AddressOfResponse {
    pub owner: String,
    // pub contract_address: Option<String>,
    // pub validator_address: Option<String>,
}

pub fn address_of(
    contract: Cw721MetadataContract,
    deps: Deps,
    token_id: String,
) -> StdResult<AddressOfResponse> {
    let token = contract.tokens.load(deps.storage, &token_id)?;
    Ok(AddressOfResponse {
        owner: token.owner.to_string(),
    })
}
