extern crate deadpool_r2d2;
extern crate r2d2_postgres;

use std::env;

use deadpool_r2d2::{Manager, Pool, Runtime};
use r2d2_postgres::postgres::{Config, Error, NoTls};
use r2d2_postgres::PostgresConnectionManager;

pub type PgManager = Manager<PostgresConnectionManager<NoTls>>;

pub type PgPool = Pool<PgManager>;

pub fn create_pool(app_name: &str) -> PgPool {
    log::info!("Loading database configuration");
    let mut config = Config::new();
    config.application_name(app_name);
    config.host(&env::var("PG_HOST").expect("PG_HOST must be set"));
    config.port(
        env::var("PG_PORT")
            .unwrap_or("5432".to_string())
            .parse()
            .expect("PG_PORT must be a number"),
    );
    config.user(&env::var("PG_USER").expect("PG_USER must be set"));
    config.password(&env::var("PG_PASSWORD").expect("PG_PASSWORD must be set"));
    config.dbname(&env::var("PG_DBNAME").expect("PG_DBNAME must be set"));
    let manager = PgManager::new(
        PostgresConnectionManager::new(config, NoTls),
        Runtime::Tokio1,
    );

    log::info!("Creating database pool");
    PgPool::builder(manager)
        .max_size(16)
        .build()
        .expect("Failed to create pool")
}

pub fn get_error_code(err: &Error) -> i32 {
    match err.code() {
        Some(code) => code.code().parse().unwrap_or(0),
        None => 0,
    }
}
