use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateTelephoneDTO {
    pub telephone_number: i32,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateTelephoneDTO {
    pub telephone_number: Option<i32>,
}
