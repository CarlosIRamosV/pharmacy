use std::error::Error;

use postgres::Row;
use postgres_from_row::FromRow;

use database::PgPool;

use crate::models::{Branch, BranchRequest, BranchUpdate, Search};
use crate::utils::rows_to_stocks;

pub async fn get_all_branches(
    pool: &PgPool,
    search: &Search,
) -> Result<Vec<Branch>, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();
    log::info!("Searching for stocks");

    let stocks = if search.name.is_some() || search.address.is_some() {
        let mut query = "SELECT * FROM branches_view WHERE".to_string();
        let mut count = 1;

        if let Some(name) = &search.name {
            query.push_str(&format!(" name = '{}'", name));
            count += 1;
        }

        if let Some(address) = &search.address {
            if count > 1 {
                query.push_str(" AND");
            }
            query.push_str(&format!(" address = '{}'", address));
        }

        conn.interact(move |client| {
            client
                .query(&query, &[])
                .map(|rows: Vec<Row>| rows_to_stocks(rows))
        })
        .await?
    } else {
        log::info!("No search parameters provided");
        conn.interact(|client| {
            client
                .query("SELECT * FROM branches_view;", &[])
                .map(|rows: Vec<Row>| rows_to_stocks(rows))
        })
        .await?
    };
    match stocks {
        Ok(stocks) => Ok(stocks),
        Err(err) => Err(Box::new(err)),
    }
}

pub async fn create_branch(
    pool: &PgPool,
    branch: BranchRequest,
) -> Result<Branch, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();
    log::info!("Creating branch");

    let branch = conn
        .interact(move |client| {
            client
                .query_one(
                    "SELECT * FROM fn_branches_insert($1, $2);",
                    &[&branch.name, &branch.address],
                )
                .map(|row| Branch::from_row(&row))
        })
        .await?;

    match branch {
        Ok(branch) => Ok(branch),
        Err(err) => Err(Box::new(err)),
    }
}

pub async fn update_branch(
    pool: &PgPool,
    id: i32,
    branch: BranchUpdate,
) -> Result<Branch, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();
    let branch = conn
        .interact(move |client| {

            // Generate the query
            let mut query = "UPDATE branches SET".to_string();
            let mut count = 1;

            if let Some(name) = &branch.name {
                query.push_str(&format!(" name = '{}'", name));
                count += 1;
            }

            if let Some(address) = &branch.address {
                if count > 1 {
                    query.push_str(",");
                }
                query.push_str(&format!(" address = '{}'", address));
            }

            query.push_str(", updated_at = now()");

            query.push_str(&format!(" WHERE id = {}", id));


            query.push_str(" RETURNING *;");

            client
                .query_one(&query, &[])
                .map(|row| Branch::from_row(&row))

        })
        .await?;

    match branch {
        Ok(branch) => Ok(branch),
        Err(err) => Err(Box::new(err)),
    }
}

pub async fn get_branch(pool: &PgPool, id: i32) -> Result<Branch, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();
    let branch = conn
        .interact(move |client| {
            client
                .query_one("SELECT * FROM branches_view WHERE id = $1;", &[&id])
                .map(|row| Branch::from_row(&row))
        })
        .await?;
    match branch {
        Ok(branch) => Ok(branch),
        Err(err) => Err(Box::new(err)),
    }
}

pub async fn delete_branch(pool: &PgPool, id: i32) -> Result<(), Box<dyn Error>> {
    let conn = pool.get().await.unwrap();
    conn.interact(move |client| {
        client
            .execute("DELETE FROM branches WHERE id = $1;", &[&id])
            .map(|_| ())
    })
        .await?.expect("Failed to delete branch");
    Ok(())

}