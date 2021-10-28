use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateEmployeeDTO {
    pub first_name: String,
    pub last_name: String,
    pub position: String,
    pub auth: i32,
    pub address: i32,
    pub contact_number: i32,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateEmployeeDTO {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub position: Option<String>,
    pub auth: Option<i32>,
    pub address: Option<i32>,
    pub contact_number: Option<i32>,
}
