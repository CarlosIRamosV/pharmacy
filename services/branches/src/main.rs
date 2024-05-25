use common::server::start;

mod actions;
mod models;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    start("Branches Service", routes::load_routes).await
}
