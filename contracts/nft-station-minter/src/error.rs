use cosmwasm_std::StdError;
use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("InvalidNumTokens {max}, min: 1")]
    InvalidNumTokens { max: u32, min: u32 },

    #[error("InvalidMaxTokensPerBatchMint {max}, min: 1")]
    InvalidMaxTokensPerBatchMint { max: u32, min: u32 },

    #[error("InvalidMaxTokensPerBatchTransfer {max}, min: 1")]
    InvalidMaxTokensPerBatchTransfer { max: u32, min: u32 },

    #[error("Instantiate cw721 error")]
    InstantiateCW721Error {},

    #[error("Invalid reply ID")]
    InvalidReplyID {},

    #[error("Invalid token id")]
    InvalidTokenId {},

    #[error("Token id: {token_id} already sold")]
    TokenIdAlreadySold { token_id: u32 },

    #[error("Sold out")]
    SoldOut {},

    #[error("Max minting limit per transaction exceeded")]
    MaxPerTxLimitExceeded {},

    #[error("Invalid base token URI (must be an IPFS URI)")]
    InvalidBaseTokenURI {},
}

impl From<ParseError> for ContractError {
    fn from(_err: ParseError) -> ContractError {
        ContractError::InvalidBaseTokenURI {}
    }
}
