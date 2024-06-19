use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn get_postgres_pool(db_uri: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(db_uri)
        .await
}

pub async fn migrate(pool: &Pool<Postgres>) {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .expect("Failed to migrate database");
}
