use common::server::start;

mod actions;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    start("Images Service", routes::load_routes).await
}
