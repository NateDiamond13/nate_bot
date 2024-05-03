use crate::prelude::{Error, Result};

use dotenvy::dotenv;
use std::env;

pub struct EnvVariables {
    pub command_prefix: String,
    pub custom_status: String,
    pub discord_token: String,
    pub reaction_target_ids: Vec<u64>,
    pub reaction_target_odds: u32,
    pub shard_count: u32,
}

pub fn load_env() -> Result<EnvVariables> {
    if dotenv().is_err() {
        println!("No .env file found. Attempting to load environment...");
    }
    Ok(EnvVariables {
        command_prefix: load_var_string("COMMAND_PREFIX")?,
        custom_status: load_var_string("CUSTOM_STATUS")?,
        discord_token: load_var_string("DISCORD_TOKEN")?,
        reaction_target_ids: load_vec_u64("REACTION_TARGET_IDS")?,
        reaction_target_odds: load_var_u32("REACTION_TARGET_ODDS", 1, u32::MAX)?,
        shard_count: load_var_u32("SHARD_COUNT", 1, 10)?,
    })
}

fn load_var_string(key: &str) -> Result<String> {
    env::var(key).map_err(|_| Error::MissingVar(key.to_string()))
}

fn load_var_u32(key: &str, min: u32, max: u32) -> Result<u32> {
    let value: u32 = env::var(key)
        .map_err(|_| Error::MissingVar(key.to_string()))?
        .parse()
        .map_err(|_| Error::MissingVar(key.to_string()))?;
    if value < min || value > max {
        return Err(Error::InvalidRangeVar(key.to_string(), min, max));
    }
    Ok(value)
}

fn load_vec_u64(key: &str) -> Result<Vec<u64>> {
    let var_string = env::var(key).map_err(|_| Error::MissingVar(key.to_string()))?;
    let results: Vec<u64> = var_string
        .split(',')
        .filter_map(|s| match s.trim().parse() {
            Ok(res) => Some(res),
            Err(_) => None,
        })
        .collect();
    Ok(results)
}
