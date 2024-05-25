use chrono::NaiveDateTime;
use postgres_from_row::FromRow;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct Search {
    pub name: Option<String>,
    pub min_price: Option<Decimal>,
    pub max_price: Option<Decimal>,
    pub branch_id: Option<i32>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct Request {
    pub name: String,
    pub description: String,
    pub price: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct Update {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<Decimal>,
}
