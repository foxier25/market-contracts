use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Decimal, StdResult, Storage};
use cosmwasm_storage::{singleton, singleton_read};

static KEY_CONFIG: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: CanonicalAddr,
    pub borrow_rate: Decimal,
}

pub fn store_config<S: Storage>(storage: &mut S, config: &Config) -> StdResult<()> {
    singleton(storage, KEY_CONFIG).save(config)
}

pub fn read_config<S: Storage>(storage: &S) -> StdResult<Config> {
    singleton_read(storage, KEY_CONFIG).load()
}
