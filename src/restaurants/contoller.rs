use std::collections::HashMap;

use axum::{extract::Query, Json};
use http::StatusCode;
use mongodb::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::model::Restaurant;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRestaurantPayload {
    pub name: String,
    pub kind: Option<String>
}

pub async fn get_restaurant(Query(params): Query<HashMap<String, String>>) -> Result<Json<Value>, StatusCode> {

    let mut got_kind = false;

    let kind = match params.get("kind") {
        Some(kind) => kind.to_string(),
        None => {
            got_kind = true;
            "".to_string()
        }
    };
    let name = match params.get("name") {
        Some(name) => name.to_string(),
        None => "".to_string()
    };
    let restaurants: Result<Vec<Restaurant>, Error>;

    if got_kind {
        restaurants = Restaurant::find_by_name(name).await;
    } else {
        restaurants = Restaurant::find_by_kind(name, kind).await;
    }


    match restaurants {
        Ok(restaurants) => {
            // let mut vec_response = Vec::new();
            // for restaurant in restaurants.iter() {
            //     vec_response.push(restaurant.serialize().unwrap());
            // }
            Ok(Json(serde_json::to_value(&restaurants).unwrap()))
            
        },
        Err(_) => return Err(StatusCode::NOT_FOUND),
    }
}

pub async fn new_restaurant(Json(restaurant): Json<Restaurant>) -> Result<Json<Value>, StatusCode> {
    match restaurant.save().await {
        Ok(_) => Ok(Json(serde_json::json!({"message": "Restaurant created"}))),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
