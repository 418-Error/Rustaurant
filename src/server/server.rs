use axum::Json;
use serde::Deserialize;
use serde_json::{
    json,
    Value,
};
use crate::models::restaurant::restaurant::Restaurant;

pub async fn lgbt() -> String {
    return "Hello, World!, this is the lgbt route".to_string();
}

pub async fn wifi() -> String {
    return "Hello, World!, this is the wifi route".to_string();
}

pub async fn smoker() -> String {
    return "Hello, World!, this is the smoker route".to_string();
}

pub async fn add_restaurant(Json(payload): Json<CreateRestaurantPayload>) -> Json<Value> {
    // Create a new restaurant
    // Return the restaurant as JSON
    let restaurant = Restaurant::create(payload).await;
    println!("{:?}", restaurant);
    return Json(json!(restaurant));
}

#[derive(Deserialize)]
pub struct CreateRestaurantPayload {
    pub name: String,
}
