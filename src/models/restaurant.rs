use serde::{Deserialize, Serialize};

use crate::server::server::CreateRestaurantPayload;

#[derive(Debug, Deserialize, Serialize)]
pub struct Restaurant {
    pub name: String,
    pub lgbt: bool,
    pub wifi: bool,
    pub smoker: bool,
    pub id: i32,
}

impl Restaurant {
    pub async fn create(payload: CreateRestaurantPayload) -> Self {
        Self {
            name: payload.name,
            id: 1,
            lgbt: true,
            wifi: true,
            smoker: true,
        }
    }
}
