//! Database library to handle connections for Postgres database using [`sqlx`].

pub mod auth_tokens;
pub mod patch_notes;
pub mod patch_notes_subscriptions;
pub mod pictures;
mod prelude;

use std::str::FromStr;
use std::time::Duration;

use log::LevelFilter;
pub use prelude::*;
use sqlx::ConnectOptions;
use sqlx::postgres::PgConnectOptions;

const STATEMENT_TIMEOUT: &str = "10min";

#[derive(Clone, Debug)]
pub struct DbPool {
    inner_pool: DbConnectionPool,
}

impl DbPool {
    pub async fn new(database_url: impl Into<String>) -> Result<Self> {
        let options = PgConnectOptions::from_str(&database_url.into())?
            .log_slow_statements(LevelFilter::Warn, Duration::from_secs(10))
            .options([("statement_timeout", STATEMENT_TIMEOUT)]);

        let inner_pool = DbConnectionPool::connect_with(options).await?;
        log::info!("Database connection pool created");
        Ok(Self { inner_pool })
    }

    pub fn from_connection_pool(pool: DbConnectionPool) -> Self {
        log::info!("Database connection pool created");
        Self { inner_pool: pool }
    }

    pub async fn get_connection(&self) -> Result<DbConnection> {
        let conn = self.inner_pool.acquire().await?;
        log::info!("Database pool connection acquired");
        Ok(conn)
    }

    pub async fn begin_transaction(&self) -> Result<DbTransaction> {
        let tx = self.inner_pool.begin().await?;
        log::info!("Database transaction starting...");
        Ok(tx)
    }
}

pub async fn commit_transaction(tx: DbTransaction) -> Result<()> {
    tx.commit().await?;
    log::info!("Database transaction committed successfully");
    Ok(())
}
