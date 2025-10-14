//! Library - Database prelude

/// Database library error
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

/// Database library result
pub type Result<T> = core::result::Result<T, Error>;

pub type DbConnectionPool = sqlx::PgPool;
pub type DbConnection = sqlx::pool::PoolConnection<sqlx::Postgres>;
pub type DbTransaction = sqlx::Transaction<'static, sqlx::Postgres>;
pub type DbQueryResult = sqlx::postgres::PgQueryResult;

pub trait DbExecutor<'a>: sqlx::PgExecutor<'a> {}
impl<'a, T: sqlx::PgExecutor<'a>> DbExecutor<'a> for T {}
