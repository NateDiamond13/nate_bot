use crate::prelude::{Error, Result};
use dotenv::dotenv;
use std::env;

pub struct EnvVariables {
    pub discord_token: String,
}

pub fn load_env() -> Result<EnvVariables> {
    if dotenv().is_err() {
        println!("No .env file found. Attempting to load environment...");
    }
    let discord_token = load_var("DISCORD_TOKEN")?;

    Ok(EnvVariables { discord_token })
}

fn load_var(key: &str) -> Result<String> {
    env::var(key).map_err(|_| Error::MissingVar(key.to_string()))
}
