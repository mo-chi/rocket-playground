use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct IndexResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: u64,
    pub name: String,
}
