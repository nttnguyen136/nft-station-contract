#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo, Order, Reply,
    ReplyOn, Response, StdResult, SubMsg, WasmMsg,
};

use crate::{
    msg::{ConfigResponse, ExecuteMsg, QueryMsg},
    state::{Config, CONFIG, CW721_ADDRESS, MINTABLE_NUM_TOKENS, MINTABLE_TOKEN_IDS},
};
use cw2::set_contract_version;
use cw_utils::parse_reply_instantiate_data;
use url::Url;

use crate::error::ContractError;
use crate::msg::InstantiateMsg;
use crate::{Extension, Metadata};

// CW721
use cw721_base::{ExecuteMsg as Cw721ExecuteMsg, InstantiateMsg as CW721InstantiateMsg, MintMsg};
pub type Cw721MetadataContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty>;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:nft-station-minter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// governance parameters
pub(crate) const MAX_TOKEN_LIMIT: u32 = 1000;
pub(crate) const MAX_TOKEN_PER_BATCH: u32 = 20;
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
    if msg.max_tokens_per_batch == 0 || msg.max_tokens_per_batch > MAX_TOKEN_PER_BATCH {
        return Err(ContractError::InvalidMaxTokensPerBatchMint {
            min: 1,
            max: MAX_TOKEN_PER_BATCH,
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
        max_tokens_per_batch: msg.max_tokens_per_batch,
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
}

pub fn execute_mint_sender(
    deps: DepsMut,
    info: MessageInfo,
    token_id: u32,
) -> Result<Response, ContractError> {
    let recipient = info.sender.clone();
    _execute_mint(deps, info, Some(recipient), Some(token_id))
}

pub fn execute_mint_to(
    deps: DepsMut,
    info: MessageInfo,
    recipient: String,
    token_id: u32,
) -> Result<Response, ContractError> {
    let recipient = deps.api.addr_validate(&recipient)?;
    _execute_mint(deps, info, Some(recipient), Some(token_id))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint { token_id } => execute_mint_sender(deps, info, token_id),
        ExecuteMsg::MintTo {
            token_id,
            recipient,
        } => execute_mint_to(deps, info, recipient, token_id),
        ExecuteMsg::BatchMint { token_ids } => execute_batch_mint_sender(deps, info, token_ids),
        ExecuteMsg::TransferNft {
            recipient,
            token_id,
        } => execute_transfer_nft(deps, info, recipient, token_id),
        ExecuteMsg::BatchTransferNft {
            recipient,
            token_ids,
        } => execute_batch_transfer_nft(deps, info, recipient, token_ids),
    }
}

fn _execute_mint(
    deps: DepsMut,
    info: MessageInfo,
    recipient: Option<Addr>,
    token_id: Option<u32>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let recipient_addr = match recipient {
        Some(some_recipient) => some_recipient,
        None => info.sender.clone(),
    };

    let mintable_token_id = match token_id {
        Some(token_id) => {
            if token_id == 0 || token_id > config.max_tokens {
                return Err(ContractError::InvalidTokenId {});
            }
            // If token_id not on mintable map, throw err
            if !MINTABLE_TOKEN_IDS.has(deps.storage, token_id) {
                return Err(ContractError::TokenIdAlreadySold { token_id });
            }
            token_id
        }

        None => {
            let mintable_tokens_result: StdResult<Vec<u32>> = MINTABLE_TOKEN_IDS
                .keys(deps.storage, None, None, Order::Ascending)
                .take(1)
                .collect();
            let mintable_tokens = mintable_tokens_result?;
            if mintable_tokens.is_empty() {
                return Err(ContractError::SoldOut {});
            }
            mintable_tokens[0]
        }
    };

    let mut msgs: Vec<CosmosMsg<Empty>> = vec![];
    let msg = _create_cw721_mint(&config, &recipient_addr, mintable_token_id);
    let msg_rs = match msg {
        Ok(msg) => msg,
        Err(ctr_err) => return Err(ctr_err),
    };
    msgs.append(&mut vec![msg_rs]);

    // Remove mintable token id from map
    MINTABLE_TOKEN_IDS.remove(deps.storage, mintable_token_id);
    let mintable_num_tokens = MINTABLE_NUM_TOKENS.load(deps.storage)?;
    // Decrement mintable num tokens
    MINTABLE_NUM_TOKENS.save(deps.storage, &(mintable_num_tokens - 1))?;

    Ok(Response::new()
        .add_attribute("sender", info.sender)
        .add_attribute("recipient", recipient_addr)
        .add_attribute("token_id", mintable_token_id.to_string())
        .add_messages(msgs))
}

fn _execute_batch_mint(
    deps: DepsMut,
    info: MessageInfo,
    recipient: Option<Addr>,
    mut batch_token_ids: Vec<u32>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let recipient_addr = match recipient {
        Some(some_recipient) => some_recipient,
        None => info.sender.clone(),
    };
    let mut count: u32 = 0;
    let mut minted_token_ids: Vec<u32> = vec![];
    let mut msgs: Vec<CosmosMsg<Empty>> = vec![];
    while let Some(token_id) = batch_token_ids.pop() {
        if count >= config.max_tokens_per_batch {
            break;
        }

        if token_id == 0 || token_id > config.max_tokens {
            return Err(ContractError::InvalidTokenId {});
        }
        // If token_id not on mintable map, throw err
        if !MINTABLE_TOKEN_IDS.has(deps.storage, token_id) {
            return Err(ContractError::TokenIdAlreadySold { token_id });
        }

        let msg = _create_cw721_mint(&config, &recipient_addr, token_id);
        let msg_rs = match msg {
            Ok(msg) => msg,
            Err(ctr_err) => return Err(ctr_err),
        };
        msgs.append(&mut vec![msg_rs]);

        // Remove mintable token id from map
        MINTABLE_TOKEN_IDS.remove(deps.storage, token_id);
        let mintable_num_tokens = MINTABLE_NUM_TOKENS.load(deps.storage)?;
        // Decrement mintable num tokens
        MINTABLE_NUM_TOKENS.save(deps.storage, &(mintable_num_tokens - 1))?;

        minted_token_ids.append(&mut vec![token_id]);
        count += 1;
    }
    let minted_token_ids_str = format!("{:?}", minted_token_ids);
    Ok(Response::new()
        .add_attribute("sender", info.sender)
        .add_attribute("recipient", recipient_addr)
        .add_attribute("token_id", minted_token_ids_str)
        .add_messages(msgs))
}

pub fn execute_batch_mint_sender(
    deps: DepsMut,
    info: MessageInfo,
    token_ids: Vec<u32>,
) -> Result<Response, ContractError> {
    let recipient = info.sender.clone();
    _execute_batch_mint(deps, info, Some(recipient), token_ids)
}

pub fn execute_transfer_nft(
    deps: DepsMut,
    info: MessageInfo,
    recipient: String,
    token_id: u32,
) -> Result<Response, ContractError> {
    let recipient = deps.api.addr_validate(&recipient)?;
    _execute_transfer_nft(deps, info, recipient, token_id)
}

pub fn execute_batch_transfer_nft(
    deps: DepsMut,
    info: MessageInfo,
    recipient: String,
    token_ids: Vec<u32>,
) -> Result<Response, ContractError> {
    let recipient = deps.api.addr_validate(&recipient)?;
    _execute_batch_transfer_nft(deps, info, recipient, token_ids)
}

fn _execute_transfer_nft(
    deps: DepsMut,
    info: MessageInfo,
    recipient: Addr,
    token_id: u32,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let mut msgs: Vec<CosmosMsg<Empty>> = vec![];
    let msg = _create_cw721_transfer(&config, &recipient, token_id);
    let msg_rs = match msg {
        Ok(msg) => msg,
        Err(ctr_err) => return Err(ctr_err),
    };
    msgs.append(&mut vec![msg_rs]);

    Ok(Response::new()
        .add_attribute("sender", info.sender)
        .add_attribute("recipient", recipient)
        .add_attribute("token_id", token_id.to_string())
        .add_messages(msgs))
}

fn _execute_batch_transfer_nft(
    deps: DepsMut,
    info: MessageInfo,
    recipient: Addr,
    mut batch_token_ids: Vec<u32>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let mut msgs: Vec<CosmosMsg<Empty>> = vec![];
    let mut count: u32 = 0;
    let mut minted_token_ids: Vec<u32> = vec![];
    while let Some(token_id) = batch_token_ids.pop() {
        if count >= config.max_tokens_per_batch {
            break;
        }

        let msg = _create_cw721_transfer(&config, &recipient, token_id);
        let msg_rs = match msg {
            Ok(msg) => msg,
            Err(ctr_err) => return Err(ctr_err),
        };
        msgs.append(&mut vec![msg_rs]);

        minted_token_ids.append(&mut vec![token_id]);
        count += 1;
    }
    let transferred_token_ids_str = format!("{:?}", minted_token_ids);
    Ok(Response::new()
        .add_attribute("sender", info.sender)
        .add_attribute("recipient", recipient)
        .add_attribute("token_id", transferred_token_ids_str)
        .add_messages(msgs))
}

fn _create_cw721_transfer<'a>(
    config: &'a Config,
    recipient_addr: &'a Addr,
    token_id: u32,
) -> Result<CosmosMsg, ContractError> {
    let transfer_msg: Cw721ExecuteMsg<Empty> = Cw721ExecuteMsg::TransferNft {
        recipient: recipient_addr.to_string(),
        token_id: token_id.to_string(),
    };
    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.cw721_address.as_ref().unwrap().to_string(),
        msg: to_binary(&transfer_msg)?,
        funds: vec![],
    });
    Ok(msg)
}

fn _create_cw721_mint<'a>(
    config: &'a Config,
    recipient_addr: &'a Addr,
    mintable_token_id: u32,
) -> Result<CosmosMsg, ContractError> {
    let mint_msg = Cw721ExecuteMsg::Mint(MintMsg::<Extension> {
        token_id: mintable_token_id.to_string(),
        owner: recipient_addr.to_string(),
        token_uri: Some(format!(
            "{}/{}",
            config.base_token_uri,
            mintable_token_id.clone()
        )),
        extension: Some(Metadata {
            ..Metadata::default()
        }),
    });
    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.cw721_address.as_ref().unwrap().to_string(),
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    });
    Ok(msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
        _ => Cw721MetadataContract::default().query(deps, env, msg.into()),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: config.owner,
        cw721_code_id: config.cw721_code_id,
        cw721_address: config.cw721_address,
        max_tokens: config.max_tokens,
        max_tokens_per_mint: config.max_tokens_per_batch,
        max_tokens_per_batch_transfer: config.max_tokens_per_batch,
        name: config.name,
        symbol: config.symbol,
        base_token_uri: config.base_token_uri,
        extension: Some(Metadata {
            ..Metadata::default()
        }),
    })
}

// Reply callback triggered from cw721 contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;
    if msg.id != INSTANTIATE_CW721_REPLY_ID {
        return Err(ContractError::InvalidReplyID {});
    }

    let reply = parse_reply_instantiate_data(msg);
    match reply {
        Ok(res) => {
            config.cw721_address = Addr::unchecked(res.contract_address.clone()).into();
            CONFIG.save(deps.storage, &config)?;
            CW721_ADDRESS.save(deps.storage, &Addr::unchecked(res.contract_address))?;
            Ok(Response::default().add_attribute("action", "instantiate_cw721_reply"))
        }
        Err(_) => Err(ContractError::InstantiateCW721Error {}),
    }
}
