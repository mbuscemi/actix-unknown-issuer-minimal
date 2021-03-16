use sqlx::postgres::{ PgPoolOptions, PgPool };

use crate::settings::Settings;

pub async fn create_pool(settings: &Settings) -> PgPool {
    PgPoolOptions::new()
        .max_connections(settings.db_max_connections)
        .connect(settings.db_connection_string().as_str())
        .await
        .expect("unable to create database pool")
}