//! Library - Database prelude

/// Database library error
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Error for failing to parse an integer from a string
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    /// Error from [`sqlx`] crate
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

/// Database library result
pub type Result<T> = core::result::Result<T, Error>;

/// Type alias for [`sqlx::PgPool`]
pub type DbConnectionPool = sqlx::PgPool;
/// Type alias for [`sqlx::pool::PoolConnection`] with [`sqlx::Postgres`]
pub type DbConnection = sqlx::pool::PoolConnection<sqlx::Postgres>;
/// Type alias for [`sqlx::Transaction`] with [`sqlx::Postgres`]
pub type DbTransaction = sqlx::Transaction<'static, sqlx::Postgres>;
/// Type alias for [`sqlx::postgres::PgQueryResult`]
pub type DbQueryResult = sqlx::postgres::PgQueryResult;

/// Trait alias for [`sqlx::PgExecutor`]
pub trait DbExecutor<'a>: sqlx::PgExecutor<'a> {}
impl<'a, T: sqlx::PgExecutor<'a>> DbExecutor<'a> for T {}
