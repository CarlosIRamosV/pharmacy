use server::server;

mod actions;
mod models;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server("Branches Service", routes::load_routes).await
}
