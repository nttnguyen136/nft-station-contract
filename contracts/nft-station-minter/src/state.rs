use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub base_token_uri: String,
    pub max_tokens: u32,
    pub max_tokens_per_batch: u32,
    pub cw721_code_id: u64,
    pub cw721_address: Option<Addr>,
    pub name: String,
    pub symbol: String,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const MINTABLE_TOKEN_IDS: Map<u32, bool> = Map::new("mintable_ids");
pub const MINTABLE_NUM_TOKENS: Item<u32> = Item::new("mintable_num_tokens");
pub const CW721_ADDRESS: Item<Addr> = Item::new("cw721_address");
