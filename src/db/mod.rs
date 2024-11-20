pub mod model;
pub mod queries;
use sqlx::{PgPool, postgres::PgPoolOptions};
use crate::config;

pub async fn connect() -> Result<PgPool, sqlx::Error> {
    let database_url = config::get_database_url();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
