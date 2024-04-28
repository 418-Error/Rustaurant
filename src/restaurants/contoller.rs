use std::collections::HashMap;
use crate::auth::auth::verify_token;

use axum::{extract::Query, Json};
use http::{HeaderMap, StatusCode};
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
            Ok(Json(serde_json::to_value(&restaurants).unwrap()))
            
        },
        Err(_) => return Err(StatusCode::NOT_FOUND),
    }
}

pub async fn new_restaurant(headers: HeaderMap, Json(restaurant): Json<Restaurant>) -> Result<Json<Value>, StatusCode> {
    let auth = match headers.get(http::header::AUTHORIZATION) {
        Some(auth) => auth.to_str().unwrap(),
        None => return Err(StatusCode::UNAUTHORIZED),
    };
    let auth = match auth.split_whitespace().last() {
        Some(auth) => auth,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let username = match verify_token(auth) {
        Ok(username) => username,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    println!("{:?}", username);

    match restaurant.save(username).await {
        Ok(_) => Ok(Json(serde_json::json!({"message": "Restaurant created"}))),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


pub async fn delete_restaurant(Json(restaurant): Json<Restaurant>) -> Result<Json<Value>, StatusCode> {
    match restaurant.delete().await {
        Ok(result) => {
            println!("Restaurant deleted {:?}", result);
            if result.deleted_count == 0 {
                return Err(StatusCode::NOT_FOUND);
            }
            Ok(Json(serde_json::json!({"message": "Restaurant deleted"})))
        },
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}