use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateAuthDTO {
    pub username: String,
    pub salt: String,
    pub verifier: Vec<u8>,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateAuthDTO {
    pub username: Option<String>,
    pub salt: Option<String>,
    pub verifier: Option<Vec<u8>>,
}
