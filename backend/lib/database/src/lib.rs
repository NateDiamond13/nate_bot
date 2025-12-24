//! Database library to handle connections for Postgres database using [`sqlx`].

/// Database functions for patch notes
pub mod patch_notes;
/// Database functions for patch notes subscriptions
pub mod patch_notes_subscriptions;
/// Database functions for pictures
pub mod pictures;
mod prelude;

use std::str::FromStr;
use std::time::Duration;

use log::LevelFilter;
pub use prelude::*;
use sqlx::ConnectOptions;
use sqlx::postgres::PgConnectOptions;

const STATEMENT_TIMEOUT: &str = "10min";

/// Wrapper around a [`DbConnectionPool`]
#[derive(Clone, Debug)]
pub struct DbPool {
    inner_pool: DbConnectionPool,
}

impl DbPool {
    /// Create a new [`DbPool`] using a `database_url`
    pub async fn new(database_url: impl Into<String>) -> Result<Self> {
        let options = PgConnectOptions::from_str(&database_url.into())?
            .log_slow_statements(LevelFilter::Warn, Duration::from_secs(10))
            .options([("statement_timeout", STATEMENT_TIMEOUT)]);

        let inner_pool = DbConnectionPool::connect_with(options).await?;
        log::info!("Database connection pool created");
        Ok(Self { inner_pool })
    }

    /// Attempt to acquire a [`DbConnection`] from the pool
    pub async fn get_connection(&self) -> Result<DbConnection> {
        let conn = self.inner_pool.acquire().await?;
        log::info!("Database pool connection acquired");
        Ok(conn)
    }

    /// Attempt to start a [`DbTransaction`]
    pub async fn begin_transaction(&self) -> Result<DbTransaction> {
        let tx = self.inner_pool.begin().await?;
        log::info!("Database transaction starting...");
        Ok(tx)
    }
}

/// Attempt to commit a [`DbTransaction`] to the database
pub async fn commit_transaction(tx: DbTransaction) -> Result<()> {
    tx.commit().await?;
    log::info!("Database transaction committed successfully");
    Ok(())
}
