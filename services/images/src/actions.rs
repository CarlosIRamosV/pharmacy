use std::error::Error;

use postgres_from_row::FromRow;

use database::PgPool;

use crate::models::{Image, Request};

pub async fn get(pool: &PgPool, id: i32) -> Result<Image, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();
    let branch = conn
        .interact(move |client| {
            client
                .query_one("SELECT * FROM images WHERE id = $1;", &[&id])
                .map(|row| Image::from_row(&row))
        })
        .await?;
    match branch {
        Ok(branch) => Ok(branch),
        Err(err) => Err(Box::new(err)),
    }
}

pub async fn create(pool: &PgPool, request: Request) -> Result<Image, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();

    let branch = conn
        .interact(move |client| {
            client
                .query_one(
                    "INSERT INTO images (image, hash) VALUES ($1, $2) RETURNING *;",
                    &[&request.get_image(), &request.hash()],
                )
                .map(|row| Image::from_row(&row))
        })
        .await?;
    match branch {
        Ok(branch) => Ok(branch),
        Err(err) => Err(Box::new(err)),
    }
}
