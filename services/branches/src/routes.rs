use actix_web::web::Query;
use actix_web::{delete, get, post, put, web};

use database::PgPool;

use crate::actions;
use crate::actions::get_all_branches;
use crate::models::{Branch, BranchRequest, BranchUpdate, Search};

///
/// Load the routes for the application
///
pub fn load_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_branches);
    cfg.service(get_branch);
    cfg.service(create_branch);
    cfg.service(update_branch);
    cfg.service(delete_branch);
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

#[post("/")]
async fn create_branch(
    pool: web::Data<PgPool>,
    branch: web::Json<BranchRequest>,
) -> Result<web::Json<Branch>, actix_web::Error> {
    log::info!("Creating branch");

    match actions::create_branch(&pool, branch.into_inner()).await {
        Ok(branch) => Ok(web::Json(branch)),
        Err(err) => {
            log::error!("Failed to create branch: {}", err);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to create branch",
            ))
        }
    }
}

#[get("/{id}")]
async fn get_branch(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> Result<web::Json<Branch>, actix_web::Error> {
    log::info!("Getting branch");

    match actions::get_branch(&pool, id.into_inner()).await {
        Ok(branch) => Ok(web::Json(branch)),
        Err(err) => {
            log::error!("Failed to get branch: {}", err);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to get branch",
            ))
        }
    }
}

#[put("/{id}")]
async fn update_branch(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    branch: web::Json<BranchUpdate>,
) -> Result<web::Json<Branch>, actix_web::Error> {
    log::info!("Updating branch");

    match actions::update_branch(&pool, id.into_inner(), branch.into_inner()).await {
        Ok(branch) => Ok(web::Json(branch)),
        Err(err) => {
            log::error!("Failed to update branch: {}", err);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to update branch",
            ))
        }
    }
}

#[delete("/{id}")]
async fn delete_branch(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> Result<web::Json<()>, actix_web::Error> {
    log::info!("Deleting branch");

    match actions::delete_branch(&pool, id.into_inner()).await {
        Ok(_) => Ok(web::Json(())),
        Err(err) => {
            log::error!("Failed to delete branch: {}", err);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to delete branch",
            ))
        }
    }
}
