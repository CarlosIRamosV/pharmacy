use chrono::NaiveDateTime;
use postgres_from_row::FromRow;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image: Option<Vec<u8>>,
    pub price: Decimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Product {
    pub fn from_row(row: &postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            image: row.get("image"),
            price: row.get("price"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }

    pub fn to_public(&self) -> Public {
        Public {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            image: self.get_image(),
            price: self.price,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }

    pub fn get_image(&self) -> String {
        match &self.image {
            Some(_image) => String::from_utf8(self.image.clone().unwrap()).unwrap(),
            None => "".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Public {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image: String,
    pub price: Decimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct Search {
    pub name: Option<String>,
    pub description: Option<String>,
    pub min_price: Option<Decimal>,
    pub max_price: Option<Decimal>,
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
