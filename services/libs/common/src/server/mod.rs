use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::{Data, Json, ServiceConfig};
use actix_web::{get, App, HttpServer};

use database::create_pool;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Health {
    status: String,
}

#[get("/health")]
async fn health() -> Json<Health> {
    Json(Health {
        status: "UP".to_string(),
    })
}

pub async fn start(app_name: &str, routes: fn(&mut ServiceConfig)) -> std::io::Result<()> {
    // Load environment variables
    crate::load_env();

    // Initialize logger
    crate::init_logger();

    // Create the database pool
    let pool = create_pool(app_name);

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

            // Health check route
            .service(health)

            // Load the routes
            .configure(routes)
    })
    .bind(crate::load_bind())?
    .bind(crate::load_bind_ipv6())?
    .run()
    .await
}
