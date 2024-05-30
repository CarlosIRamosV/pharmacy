use std::hash::{DefaultHasher, Hasher};
use std::ptr::hash;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub product_id: i32,
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
