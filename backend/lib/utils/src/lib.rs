//! Utils library to handle environment variables, etc.

mod env;
mod prelude;

pub use env::{EnvVariables, get_env_variables};
pub use prelude::{Error, Result};
