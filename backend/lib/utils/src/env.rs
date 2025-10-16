use std::env;
use std::ops::Deref;
use std::sync::LazyLock;

use crate::prelude::{Error, Result};

const ENV_FILENAME: &str = ".env";
static ENV_VARIABLES: LazyLock<Result<EnvVariables>> = LazyLock::new(load_env);

#[derive(Debug, Clone)]
pub struct EnvVariables {
    pub audit_enabled_servers: Vec<u64>,
    pub custom_status: String,
    pub database_url: String,
    pub discord_token: String,
    pub lottery_odds: u32,
    pub queue_max_sounds: usize,
    pub reaction_target_ids: Vec<u64>,
    pub reaction_target_odds: u32,
    pub redis_url: String,
    pub soundcloud_client_id: String,
    pub soundcloud_client_secret: String,
    pub webdriver_port: u16,
}

/// Get the configuration variables defined in the environment (can panic)
pub fn get_config() -> EnvVariables {
    match ENV_VARIABLES.deref() {
        Ok(env_vars) => env_vars.clone(),
        Err(err) => panic!("Error occurred while reading from env: {err:?}"),
    }
}

/// Get the configuration variables defined in the environment (will not panic)
pub fn get_config_safe() -> Result<EnvVariables> {
    ENV_VARIABLES.clone()
}

fn load_env() -> Result<EnvVariables> {
    if dotenvy::from_filename_override(ENV_FILENAME).is_err() {
        log::warn!("No .env file found. Attempting to load environment...");
    }
    Ok(EnvVariables {
        audit_enabled_servers: load_vec_u64("AUDIT_ENABLED_SERVERS")?,
        custom_status: load_var_string("CUSTOM_STATUS")?,
        database_url: load_var_string("DATABASE_URL")?,
        discord_token: load_var_string("DISCORD_TOKEN")?,
        lottery_odds: load_var_u32("LOTTERY_ODDS", 1, u32::MAX)?,
        queue_max_sounds: load_var_usize("QUEUE_MAX_SOUNDS")?,
        reaction_target_ids: load_vec_u64("REACTION_TARGET_IDS")?,
        reaction_target_odds: load_var_u32("REACTION_TARGET_ODDS", 1, u32::MAX)?,
        redis_url: load_var_string("REDIS_URL")?,
        soundcloud_client_id: load_var_string("SOUNDCLOUD_CLIENT_ID")?,
        soundcloud_client_secret: load_var_string("SOUNDCLOUD_CLIENT_SECRET")?,
        webdriver_port: load_var_u16("WEBDRIVER_PORT")?,
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

fn load_var_u16(key: &str) -> Result<u16> {
    let value: u16 = env::var(key)
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
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    Ok(results)
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::env::{get_config, get_config_safe};

    #[test]
    fn load_env_vars() {
        let _ = get_config();

        let v = get_config_safe();
        assert!(v.is_ok());
    }
}
