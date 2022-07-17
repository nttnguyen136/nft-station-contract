use crate::error::ContractError;
use crate::msg::{ContractInfo, InstantiateMsg, MintMsg, UpdateMetadataMsg};
use crate::state::{CONTRACT_INFO, MINTING_FEES_INFO};
use crate::utils::username_is_valid;
use crate::Cw721MetadataContract;

use cw2::set_contract_version;
use cw721_base::state::TokenInfo;

use cosmwasm_std::{ensure_eq, DepsMut, Env, MessageInfo, Response, StdResult};

const CONTRACT_NAME: &str = "crates.io:name-service";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn execute_instantiate(
    contract: Cw721MetadataContract,
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Save contract info
    let info = ContractInfo {
        name: msg.name,
        symbol: msg.symbol,
    };
    CONTRACT_INFO.save(deps.storage, &info)?;

    MINTING_FEES_INFO.save(deps.storage, &msg.base_mint_fee)?;

    // Save minter
    let minter = deps.api.addr_validate(&msg.minter)?;
    contract.minter.save(deps.storage, &minter)?;

    Ok(Response::default())
}

pub fn mint(
    contract: Cw721MetadataContract,
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: MintMsg,
) -> Result<Response, ContractError> {
    let address_trying_to_mint = info.sender.clone();

    ensure_eq!(
        msg.owner,
        address_trying_to_mint,
        ContractError::Unauthorized {}
    );

    // validate owner addr
    let owner_address = deps.api.addr_validate(&msg.owner)?;

    // username == token_id
    // normalize it to lowercase
    let username = &msg.token_id.to_lowercase();

    if !username_is_valid(deps.as_ref(), username) {
        return Err(ContractError::TokenNameInvalid {});
    }

    // create the token
    // this will fail if token_id (i.e. username)
    // is already claimed
    let token = TokenInfo {
        owner: owner_address,
        approvals: vec![],
        token_uri: msg.token_uri,
        extension: msg.extension,
    };

    contract
        .tokens
        .update(deps.storage, username, |old| match old {
            Some(_) => Err(ContractError::Claimed {}),
            None => Ok(token),
        })?;

    contract.increment_tokens(deps.storage)?;

    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_attribute("minter", address_trying_to_mint)
        .add_attribute("token_id", msg.token_id))
}

pub fn update_metadata(
    contract: Cw721MetadataContract,
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: UpdateMetadataMsg,
) -> Result<Response, ContractError> {
    let address_trying_to_update = info.sender.clone();
    let token_id = msg.token_id.clone();
    let username_nft = contract.tokens.load(deps.storage, &token_id)?;

    let username_owner = username_nft.owner.clone();

    // check it's the owner of the NFT updating meta
    ensure_eq!(
        username_owner,
        address_trying_to_update,
        ContractError::Unauthorized {}
    );

    contract
        .tokens
        .update(deps.storage, &token_id, |token| -> StdResult<_> {
            match token {
                Some(mut nft) => {
                    nft.extension = msg.metadata;
                    Ok(nft)
                }
                None => Ok(username_nft),
            }
        })?;

    Ok(Response::new()
        .add_attribute("action", "update_metadata")
        .add_attribute("owner", info.sender)
        .add_attribute("token_id", token_id))
}
