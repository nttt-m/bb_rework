use chrono::{Date, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateEmployeeDTO {
    pub first_name: String,
    pub last_name: String,
    pub dob: NaiveDate,
    pub address: i32,
    pub contact_number: i32,
    pub position: String,
    pub authn: i32,
    pub authz: bool,
    pub pending: bool,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateEmployeeDTO {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub dob: Option<NaiveDate>,
    pub address: Option<i32>,
    pub contact_number: Option<i32>,
    pub position: Option<String>,
    pub authn: Option<i32>,
    pub authz: Option<bool>,
    pub pending: Option<bool>,
}
