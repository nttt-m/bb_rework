use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateAddressDTO {
    pub house_name_number: String,
    pub street: Option<String>,
    pub town_city: Option<String>,
    pub region: Option<String>,
    pub postal_code: String,
    pub country: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateAddressDTO {
    pub house_name_number: Option<String>,
    pub street: Option<String>,
    pub town_city: Option<String>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}
