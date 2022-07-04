#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Empty, SubMsg};

use crate::state::{Config, CONFIG, MINTABLE_NUM_TOKENS, MINTABLE_TOKEN_IDS};
use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, ReplyOn, Response, StdResult, WasmMsg,
};
use cw2::set_contract_version;
use url::Url;

use crate::error::ContractError;
use crate::msg::InstantiateMsg;
use crate::Extension;

// CW721
pub use cw721_base::{InstantiateMsg as CW721InstantiateMsg, MintMsg, MinterResponse, QueryMsg};
pub type Cw721MetadataContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension>;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:nft-station-minter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// governance parameters
pub(crate) const MAX_TOKEN_LIMIT: u32 = 10000;
pub(crate) const MAX_TOKEN_PER_BATCH_LIMIT: u32 = 20;
pub(crate) const INSTANTIATE_CW721_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // Check the number of tokens is more than zero and less than the max limit
    if msg.num_tokens == 0 || msg.num_tokens > MAX_TOKEN_LIMIT {
        return Err(ContractError::InvalidNumTokens {
            min: 1,
            max: MAX_TOKEN_LIMIT,
        });
    }

    // Check the number of tokens per batch is more than zero and less than the max limit
    if msg.max_tokens_per_batch_mint == 0
        || msg.max_tokens_per_batch_mint > MAX_TOKEN_PER_BATCH_LIMIT
    {
        return Err(ContractError::InvalidMaxTokensPerBatchMint {
            min: 1,
            max: MAX_TOKEN_PER_BATCH_LIMIT,
        });
    }

    // Check the number of tokens per batch is more than zero and less than the max limit
    if msg.max_tokens_per_batch_transfer == 0
        || msg.max_tokens_per_batch_transfer > MAX_TOKEN_PER_BATCH_LIMIT
    {
        return Err(ContractError::InvalidMaxTokensPerBatchTransfer {
            min: 1,
            max: MAX_TOKEN_PER_BATCH_LIMIT,
        });
    }

    // Check that base_token_uri is a valid IPFS uri
    let parsed_token_uri = Url::parse(&msg.base_token_uri)?;
    if parsed_token_uri.scheme() != "ipfs" {
        return Err(ContractError::InvalidBaseTokenURI {});
    }

    let config = Config {
        owner: info.sender.clone(),
        cw721_code_id: msg.cw721_code_id,
        cw721_address: None,
        name: msg.name.clone(),
        symbol: msg.symbol.clone(),
        base_token_uri: msg.base_token_uri.clone(),
        max_tokens: msg.num_tokens,
        max_tokens_per_batch_mint: msg.max_tokens_per_batch_mint,
        max_tokens_per_batch_transfer: msg.max_tokens_per_batch_transfer,
        royalty_percentage: msg.royalty_percentage,
        royalty_payment_address: msg.royalty_payment_address,
    };
    CONFIG.save(deps.storage, &config)?;
    MINTABLE_NUM_TOKENS.save(deps.storage, &msg.num_tokens)?;
    let sub_msgs: Vec<SubMsg> = vec![SubMsg {
        id: INSTANTIATE_CW721_REPLY_ID,
        msg: WasmMsg::Instantiate {
            admin: Some(info.sender.to_string()),
            code_id: msg.cw721_code_id,
            msg: to_binary(&CW721InstantiateMsg {
                name: msg.name,
                symbol: msg.symbol,
                minter: env.contract.address.to_string(),
            })?,
            funds: vec![],
            label: String::from("Check CW721"),
        }
        .into(),
        gas_limit: None,
        reply_on: ReplyOn::Success,
    }];

    // Save mintable token ids map
    for token_id in 1..=msg.num_tokens {
        MINTABLE_TOKEN_IDS.save(deps.storage, token_id, &true)?;
    }

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("contract_name", CONTRACT_NAME)
        .add_attribute("contract_version", CONTRACT_VERSION)
        .add_submessages(sub_msgs))
    // Cw721MetadataContract::default().instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
    // Cw721MetadataContract::default().execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    Cw721MetadataContract::default().query(deps, env, msg)
}
