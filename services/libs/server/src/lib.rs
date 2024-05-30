extern crate actix_web;

use std::env::var;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::{Data, ServiceConfig};
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use env_logger::init_from_env;

mod health;

pub struct AppState {
    pub app_name: String,
    pub pool: database::PgPool,
}

pub async fn server(name: &str, routes: fn(&mut ServiceConfig)) -> std::io::Result<()> {
    dotenv().ok();

    // Initialize logger
    init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_data = Data::new(AppState {
        app_name: name.to_string(),
        pool: database::create_pool(name),
    });

    log::info!("Starting server");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .service(health::health_state)
            .configure(routes)
    })
    .bind(bind_v4())?
    .bind(bind_v6())?
    .run()
    .await
}

fn bind_v4() -> String {
    let bind = var("IPV4").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    log::info!("Binding to {}", bind);
    bind
}

fn bind_v6() -> String {
    let bind = var("IPV6").unwrap_or_else(|_| "[::1]:8080".to_string());
    log::info!("Binding to {}", bind);
    bind
}
