mod env;
pub use env::load_env;

mod spam;
pub use spam::post_to_spam_channel;
