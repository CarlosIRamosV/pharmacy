use std::hash::{DefaultHasher, Hasher};
use std::ptr::hash;

use chrono::NaiveDateTime;
use postgres_from_row::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Image {
    pub id: i32,
    pub image: Vec<u8>,
    pub hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Image {
    pub fn get_image(&self) -> String {
        String::from_utf8(self.image.clone()).unwrap()
    }

    pub fn get_content_type(&self) -> String {
        let image = self.get_image();
        image.split(',').collect::<Vec<&str>>()[0]
            .split(';')
            .collect::<Vec<&str>>()[0]
            .to_string()
            .replace("data:", "")
    }

    pub fn get_content(&self) -> String {
        let image = self.get_image();
        image.split(',').collect::<Vec<&str>>()[1]
            .to_string()
            .replace("base64,", "")
            .replace(" ", "+")
    }
}

#[derive(Debug, Deserialize)]
pub struct Request {
    pub image: String,
}

impl Request {
    pub fn hash(&self) -> String {
        let mut hasher = DefaultHasher::new();
        hash(&self.image, &mut hasher);
        format!("{:x}", hasher.finish())
    }

    pub fn get_image(&self) -> Vec<u8> {
        self.image.as_bytes().to_vec()
    }
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub image: String,
}
