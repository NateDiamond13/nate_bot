mod env;
pub use env::{load_env, EnvVariables};

mod spam;
pub use spam::post_to_spam_channel;
