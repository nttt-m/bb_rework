use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LoginDTO {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterDTO {
    pub first_name: String,
    pub last_name: String,
    pub position: String,
    pub house_name_number: String,
    pub street: Option<String>,
    pub town_city: Option<String>,
    pub region: Option<String>,
    pub postal_code: String,
    pub country: String,
    pub username: String,
    pub password: String,
    pub telephone_number: String,
}