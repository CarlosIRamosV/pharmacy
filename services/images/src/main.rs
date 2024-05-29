use server::server;

mod actions;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server("Images Service", routes::load_routes).await
}
