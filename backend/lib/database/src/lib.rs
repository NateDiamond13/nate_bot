//! Database library to handle connections for Postgres database using [`sqlx`].

pub mod pictures;
mod prelude;

pub use sqlx::PgPool;

pub use prelude::*;

pub async fn get_connection_pool(database_url: &str) -> Result<PgPool> {
    PgPool::connect(database_url).await.map_err(Error::Sqlx)
}
