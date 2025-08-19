// Note: this is a file to store the types for the temporary templates

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct CreateReq {
    pub name: String,
    pub description: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct CreateRes {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct GetResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub content: String,
}
