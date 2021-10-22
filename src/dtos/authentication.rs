use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateAuthDTO {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateAuthDTO {
    pub username: Option<String>,
    pub password: Option<String>,
}
