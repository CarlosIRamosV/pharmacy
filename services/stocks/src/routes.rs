use actix_web::web::Query;
use actix_web::{delete, get, post, put, web};

use database::PgPool;

use crate::actions;
use crate::actions::get_all_stocks;
use crate::models::{Search, Stock, StockRequest, StockUpdate};

///
/// Load the routes for the application
///
pub fn load_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_stocks);
    cfg.service(get_stock);
    cfg.service(post_stock);
    cfg.service(put_stock);
    cfg.service(delete_stock);
}

#[get("/")]
async fn get_stocks(
    pool: web::Data<PgPool>,
    search: Option<Query<Search>>,
) -> Result<web::Json<Vec<Stock>>, actix_web::Error> {
    // Get search parameters
    let search: Search = search.map_or(Search::default(), |search| search.into_inner());

    log::info!("Searching for stocks");

    // Check if search parameters were provided
    log::info!("{:?}", search);

    match get_all_stocks(&pool, &search).await {
        Ok(stocks) => Ok(web::Json(stocks)),
        Err(err) => {
            log::error!("Failed to get stocks: {}", err);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to get stocks",
            ))
        }
    }
}

#[post("/")]
async fn post_stock(
    pool: web::Data<PgPool>,
    stock: web::Json<StockRequest>,
) -> Result<web::Json<Stock>, actix_web::Error> {
    match actions::create_stock(&pool, stock.into_inner()).await {
        Ok(stock) => Ok(web::Json(stock)),
        Err(err) => {
            log::error!("Failed to create stock: {}", err);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to create stock",
            ))
        }
    }
}

#[get("/{id}")]
async fn get_stock(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> Result<web::Json<Stock>, actix_web::Error> {
    match actions::get_stock(&pool, id.into_inner()).await {
        Ok(stock) => Ok(web::Json(stock)),
        Err(err) => {
            log::error!("Failed to get stock: {}", err);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to get stock",
            ))
        }
    }
}

#[put("/{id}")]
async fn put_stock(
    pool: web::Data<PgPool>,
    stock: web::Json<StockUpdate>,
    id: web::Path<i32>,
) -> Result<web::Json<Stock>, actix_web::Error> {
    match actions::update_stock(&pool, id.into_inner(), stock.into_inner()).await {
        Ok(stock) => Ok(web::Json(stock)),
        Err(err) => {
            log::error!("Failed to update stock: {}", err);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to update stock",
            ))
        }
    }
}

#[delete("/{id}")]
async fn delete_stock(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> Result<web::Json<()>, actix_web::Error> {
    match actions::delete_stock(&pool, id.into_inner()).await {
        Ok(_) => Ok(web::Json(())),
        Err(err) => {
            log::error!("Failed to delete stock: {}", err);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to delete stock",
            ))
        }
    }
}
