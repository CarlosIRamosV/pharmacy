use actix_web::{get, web};

use database::PgPool;

use crate::actions;
use crate::models::{Product, Search};

///
/// Load the routes for the application
///
pub fn load_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_products);
}

#[get("/")]
pub async fn get_products(
    pool: web::Data<PgPool>,
    search: Option<web::Query<Search>>,
) -> Result<web::Json<Vec<Product>>, actix_web::Error> {
    // Check if a search query was provided
    let products = if let Some(search) = search {
        actions::search_products(&pool, search.into_inner())
            .await
            .map_err(|e| {
                log::error!("Failed to search products: {}", e);
                actix_web::error::ErrorInternalServerError("Failed to search products")
            })?
    } else {
        actions::get_all_products(&pool).await.map_err(|e| {
            log::error!("Failed to get products: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to get products")
        })?
    };

    Ok(web::Json(products))
}
