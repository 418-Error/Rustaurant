use crate::restaurants::model::Restaurant;
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};

pub async fn lgbt() -> String {
    return "Hello, World!, this is the lgbt route".to_string();
}

pub async fn wifi() -> String {
    return "Hello, World!, this is the wifi route".to_string();
}

pub async fn smoker() -> String {
    return "Hello, World!, this is the smoker route".to_string();
}

#[derive(Deserialize)]
pub struct CreateRestaurantPayload {
    pub name: String,
}
