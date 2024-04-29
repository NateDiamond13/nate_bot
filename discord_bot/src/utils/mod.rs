use crate::prelude::{Error, Result};

use dotenvy::dotenv;
use std::env;

pub struct EnvVariables {
    pub command_prefix: String,
    pub custom_status: String,
    pub discord_token: String,
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
