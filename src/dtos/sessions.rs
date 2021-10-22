use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateSessionDTO {
    pub user_id: i32,
    pub token: String,
}
