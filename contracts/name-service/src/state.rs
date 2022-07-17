use cosmwasm_std::Uint128;
use cw_storage_plus::Item;

use crate::msg::ContractInfo;

pub const CONTRACT_INFO: Item<ContractInfo> = Item::new("contract_info");
pub const MINTING_FEES_INFO: Item<Option<Uint128>> = Item::new("minting_fees");
pub const USERNAME_LENGTH_CAP: Item<Option<Uint128>> = Item::new("minting_fees");
