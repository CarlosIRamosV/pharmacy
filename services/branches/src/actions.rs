use std::error::Error;

use postgres::Row;
use postgres_from_row::FromRow;

use database::PgPool;

use crate::models::{Branch, Request, Search, Update};
use crate::utils::rows_to_branches;

pub async fn get_all(pool: &PgPool, search: &Search) -> Result<Vec<Branch>, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();

    let branches = if search.name.is_some()
        || search.address.is_some()
        || search.limit.is_some()
        || search.offset.is_some()
    {
        let mut query = "SELECT * FROM branches_view".to_string();
        let mut count = 1;

        if search.name.is_some() || search.address.is_some() {
            query.push_str(" WHERE");
        }

        if let Some(name) = &search.name {
            query.push_str(&format!(" name LIKE '%{}%'", name));
            count += 1;
        }

        if let Some(address) = &search.address {
            if count > 1 {
                query.push_str(" AND");
            }
            query.push_str(&format!(" address LIKE '%{}%'", address));
        }

        if let Some(limit) = search.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = search.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        conn.interact(move |client| {
            client
                .query(&query, &[])
                .map(|rows: Vec<Row>| rows_to_branches(rows))
        })
        .await?
    } else {
        conn.interact(|client| {
            client
                .query("SELECT * FROM branches_view;", &[])
                .map(|rows: Vec<Row>| rows_to_branches(rows))
        })
        .await?
    };
    match branches {
        Ok(branches) => Ok(branches),
        Err(err) => Err(Box::new(err)),
    }
}

pub async fn get(pool: &PgPool, id: i32) -> Result<Branch, Box<dyn Error>> {
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

pub async fn create(pool: &PgPool, request: Request) -> Result<Branch, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();
    let branch = conn
        .interact(move |client| {
            client
                .query_one(
                    "SELECT * FROM fn_create_branch($1, $2);",
                    &[&request.name, &request.address],
                )
                .map(|row| Branch::from_row(&row))
        })
        .await?;
    match branch {
        Ok(branch) => Ok(branch),
        Err(err) => Err(Box::new(err)),
    }
}

pub async fn update(pool: &PgPool, id: i32, update: Update) -> Result<Branch, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();
    let branch = conn
        .interact(move |client| {
            let mut query = "UPDATE branches SET".to_string();
            let mut count = 1;

            if let Some(name) = &update.name {
                query.push_str(&format!(" name = '{}'", name));
                count += 1;
            }

            if let Some(address) = &update.address {
                if count > 1 {
                    query.push_str(",");
                }
                query.push_str(&format!(" address = '{}'", address));
                count += 1;
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

pub async fn delete(pool: &PgPool, id: i32) -> Result<Branch, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();
    let branch = conn
        .interact(move |client| {
            client
                .query_one("DELETE FROM branches WHERE id = $1 RETURNING *;", &[&id])
                .map(|row| Branch::from_row(&row))
        })
        .await?;
    match branch {
        Ok(branch) => Ok(branch),
        Err(err) => Err(Box::new(err)),
    }
}
