use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};

use common::{init_logger, load_bind, load_bind_ipv6, load_env};
use database::create_pool;

mod actions;
mod models;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    load_env();

    // Initialize logger
    init_logger();

    // Create the database pool
    let pool = create_pool("Stocks Service");

    // Start the server
    log::info!("Starting server");
    HttpServer::new(move || {
        App::new()
            // Enable logger
            .wrap(Logger::default())
            // Enable CORS
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            // Pass the database pool to the application
            .app_data(Data::new(pool.clone()))
            // Load the routes
            .configure(routes::load_routes)
    })
    .bind(load_bind())?
    .bind(load_bind_ipv6())?
    .run()
    .await
}
