pub mod pictures;

use sqlx::{Error, PgPool};

pub async fn get_connection_pool(database_url: &str) -> Result<PgPool, Error> {
    PgPool::connect(database_url).await
}
