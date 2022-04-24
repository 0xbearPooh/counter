use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractState {
    pub owner: Addr,
    pub asset_name: String,
}

pub const CONTRACT_STATE: Item<ContractState> = Item::new("state");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deposits {
    pub amount: Uint128,
}
pub const DEPOSITS: Map<&Addr, Deposits> = Map::new("deposits");
