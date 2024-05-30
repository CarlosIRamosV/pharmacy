use std::error::Error;

use database::{get_error_code, PgPool};

use crate::models::Request;

pub async fn create(pool: &PgPool, request: Request) -> Result<String, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();

    let branch = conn
        .interact(move |client| {
            client.query_one(
                "INSERT INTO image (product_id, image, hash) VALUES ($1, $2, $3)",
                &[&request.product_id, &request.get_image(), &request.hash()],
            )
        })
        .await
        .unwrap();
    if branch.is_err() {
        log::info!("{}", get_error_code(&branch.err().unwrap()));
    }
    Ok("Image created".to_string())
}
