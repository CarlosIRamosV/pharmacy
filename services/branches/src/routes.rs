use actix_web::{delete, get, post, put, web};
use actix_web::web::Query;

use database::PgPool;

use crate::actions;
use crate::actions::get_all_branches;
use crate::models::{Branch, Search, StockRequest, StockUpdate};

///
/// Load the routes for the application
///
pub fn load_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_branches);
}

#[get("/")]
async fn get_branches(
    pool: web::Data<PgPool>,
    search: Option<Query<Search>>,
) -> Result<web::Json<Vec<Branch>>, actix_web::Error> {
    // Get search parameters
    let search: Search = search.map_or(Search::default(), |search| search.into_inner());

    log::info!("Searching for stocks");

    // Check if search parameters were provided
    log::info!("{:?}", search);

    match get_all_branches(&pool, &search).await {
        Ok(stocks) => Ok(web::Json(stocks)),
        Err(err) => {
            log::error!("Failed to get stocks: {}", err);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to get stocks",
            ))
        }
    }
}