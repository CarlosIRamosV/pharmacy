use chrono::NaiveDateTime;
use postgres_from_row::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Branch {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Default, Deserialize)]
pub struct Search {
    pub name: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StockRequest {
    pub product_id: i32,
    pub branch_id: i32,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct StockUpdate {
    pub quantity: i32,
}
