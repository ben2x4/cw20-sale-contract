use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin, Uint128};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub cw20_address: Addr,
    pub price: Coin,
    pub balance: Uint128,
}

pub const STATE: Item<State> = Item::new("state");
