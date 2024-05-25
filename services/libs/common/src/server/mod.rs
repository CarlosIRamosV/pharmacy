use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use actix_web::web::{Data, ServiceConfig};

use database::create_pool;

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
            // Load the routes
            .configure(routes)
    })
        .bind(crate::load_bind())?
        .bind(crate::load_bind_ipv6())?
        .run()
        .await
}