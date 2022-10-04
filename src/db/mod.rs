pub mod migrations;

use sqlx::{
    postgres::PgConnectOptions, postgres::PgPoolOptions, PgPool
};

pub async fn init_pool(db: &str) -> anyhow::Result<PgPool> {
    let options = PgConnectOptions::new()
        .database(db)
        .application_name("trufel")
        .host("localhost")
        .port(5432)
        .username("trufel_app")
        .password("ze88jVyQpv2eaF");

    let pool = PgPoolOptions::new()
        .min_connections(1)
        .max_connections(5)
        .test_before_acquire(true)
        .connect_with(options)
        .await?;

    Ok(pool)
}
