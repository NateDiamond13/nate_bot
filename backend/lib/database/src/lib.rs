//! Database library to handle connections for Postgres database using [`sqlx`].

pub mod patch_notes;
pub mod patch_notes_subscriptions;
pub mod pictures;
mod prelude;

pub use sqlx::PgPool;

pub use prelude::*;

pub type PgTransaction = sqlx::Transaction<'static, sqlx::Postgres>;

pub async fn get_connection_pool(database_url: &str) -> Result<PgPool> {
    PgPool::connect(database_url).await.map_err(Error::Sqlx)
}

pub async fn get_transaction(database_url: &str) -> Result<PgTransaction> {
    let conn = PgPool::connect(database_url).await?;
    Ok(conn.begin().await?)
}
