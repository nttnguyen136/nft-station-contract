use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// base_token_uri of NFTs
    pub base_token_uri: String,
    /// number token of NFTs
    pub num_tokens: u32,
    /// max number token of NFTs can be minted a batch
    pub max_tokens_per_batch_mint: u32,
    /// max number token of NFTs can be transferred a batch
    pub max_tokens_per_batch_transfer: u32,
    /// code id of cw721 was deploy before
    pub cw721_code_id: u64,
    /// name of NFTs
    pub name: String,
    /// symbol of NFTs
    pub symbol: String,
    /// royalty percentage can be received
    pub royalty_percentage: Option<u64>,
    /// royalty addresses
    pub royalty_payment_address: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

