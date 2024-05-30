use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response {
    pub status: u16,
    pub message: String,
}
