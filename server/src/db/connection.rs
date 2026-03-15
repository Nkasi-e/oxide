use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::{config::AppConfig, error::AppResult};

pub async fn connect(config: &AppConfig) -> AppResult<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&config.database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
