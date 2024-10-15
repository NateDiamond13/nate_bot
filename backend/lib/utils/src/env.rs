use crate::prelude::{Error, Result};

use std::{env, ops::Deref, sync::LazyLock};

const ENV_FILENAME: &str = ".env.dev";
static ENV_VARIABLES: LazyLock<Result<EnvVariables>> = LazyLock::new(load_env);

#[derive(Debug, Clone)]
pub struct EnvVariables {
    pub audit_enabled_servers: Vec<u64>,
    pub command_prefix: String,
    pub custom_status: String,
    pub database_url: String,
    pub discord_token: String,
    pub lottery_odds: u32,
    pub queue_max_sounds: usize,
    pub reaction_target_ids: Vec<u64>,
    pub reaction_target_odds: u32,
}

pub fn get_env_variables() -> EnvVariables {
    match ENV_VARIABLES.deref() {
        Ok(c) => c.clone(),
        Err(e) => panic!("Error occurred while reading from env: {:?}", e),
    }
}

fn load_env() -> Result<EnvVariables> {
    if dotenvy::from_filename_override(ENV_FILENAME).is_err() {
        println!("No .env file found. Attempting to load environment...");
    }
    Ok(EnvVariables {
        audit_enabled_servers: load_vec_u64("AUDIT_ENABLED_SERVERS")?,
        command_prefix: load_var_string("COMMAND_PREFIX")?,
        custom_status: load_var_string("CUSTOM_STATUS")?,
        database_url: load_var_string("DATABASE_URL")?,
        discord_token: load_var_string("DISCORD_TOKEN")?,
        lottery_odds: load_var_u32("LOTTERY_ODDS", 1, u32::MAX)?,
        queue_max_sounds: load_var_usize("QUEUE_MAX_SOUNDS")?,
        reaction_target_ids: load_vec_u64("REACTION_TARGET_IDS")?,
        reaction_target_odds: load_var_u32("REACTION_TARGET_ODDS", 1, u32::MAX)?,
    })
}

fn load_var_string(key: &str) -> Result<String> {
    env::var(key).map_err(|_| Error::MissingVar(key.to_string()))
}

fn load_var_usize(key: &str) -> Result<usize> {
    let value: usize = env::var(key)
        .map_err(|_| Error::MissingVar(key.to_string()))?
        .parse()
        .map_err(|_| Error::MissingVar(key.to_string()))?;
    Ok(value)
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
