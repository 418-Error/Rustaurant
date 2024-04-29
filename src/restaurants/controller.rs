use crate::{api::server::AppState, auth::auth::verify_token, db::db::client};
use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Query, State},
    Json,
};
use dotenv::dotenv;
use http::{HeaderMap, StatusCode};
use mongodb::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    model::Restaurant,
    service::{get_accessible_restaurants_agg, get_restaurant_agg_user, get_sports_agg},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRestaurantPayload {
    pub name: String,
    pub kind: Option<String>,
}

pub async fn get_restaurant(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Value>, StatusCode> {
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
        None => "".to_string(),
    };
    let restaurants: Result<Vec<Restaurant>, Error>;

   
    let mut session = match state.db.start_session(None).await {
        Ok(session) => session,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    session.start_transaction(None).await.unwrap();

    if got_kind {
        restaurants = Restaurant::find_by_name(name, &mut session).await;
    } else {
        restaurants = Restaurant::find_by_kind(name, kind, &mut session).await;
    }

    let results = match restaurants {
        Ok(restaurants) => Ok(Json(serde_json::to_value(&restaurants).unwrap())),
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };

    match session.commit_transaction().await {
        Ok(_) => (),
        Err(err) => {
            println!("Error committing transaction {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    results
}

pub async fn new_restaurant(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(restaurant): Json<Restaurant>,
) -> Result<Json<Value>, StatusCode> {
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

    dotenv().ok();

    let mut session = match state.db.start_session(None).await {
        Ok(session) => session,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    session.start_transaction(None).await.unwrap();

    let results = match restaurant.save(username, &mut session).await {
        Ok(_) => Ok(Json(serde_json::json!({"message": "Restaurant created"}))),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    match session.commit_transaction().await {
        Ok(_) => (),
        Err(err) => {
            println!("Error committing transaction {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    results
}

pub async fn delete_restaurant(
    Json(restaurant): Json<Restaurant>,
) -> Result<Json<Value>, StatusCode> {
    let client = client().await;
    if let Err(err) = client {
        println!("{:?}", err);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let mut session = match client.unwrap().start_session(None).await {
        Ok(session) => session,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    session.start_transaction(None).await.unwrap();
    let results = match restaurant.delete(&mut session).await {
        Ok(result) => {
            println!("Restaurant deleted {:?}", result);
            if result.deleted_count == 0 {
                return Err(StatusCode::NOT_FOUND);
            }
            Ok(Json(serde_json::json!({"message": "Restaurant deleted"})))
        }
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    match session.commit_transaction().await {
        Ok(_) => (),
        Err(err) => {
            println!("Error committing transaction {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    results
}

pub async fn update_restaurant(
    Json(restaurant): Json<Restaurant>,
) -> Result<Json<Value>, StatusCode> {
    let client = client().await;
    if let Err(err) = client {
        println!("{:?}", err);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let mut session = match client.unwrap().start_session(None).await {
        Ok(session) => session,
        Err(err) => {
            println!("Error starting session {:}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if restaurant.id.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }

    if !restaurant.osm_timestamp.is_none()
        || !restaurant.osm_version.is_none()
        || !restaurant.osm_changeset.is_none()
        || !restaurant.osm_user.is_none()
        || !restaurant.osm_uid.is_none()
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    session.start_transaction(None).await.unwrap();
    let results = match restaurant.update(&mut session).await {
        Ok(result) => {
            println!("Restaurant updated {:?}", result);
            if result.modified_count == 0 {
                return Err(StatusCode::NOT_FOUND);
            }
            Ok(Json(serde_json::json!({"message": "Restaurant updated"})))
        }
        Err(err) => {
            println!("Error updating restaurant {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match session.commit_transaction().await {
        Ok(_) => (),
        Err(err) => {
            println!("Error committing transaction {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    results
}

pub async fn get_restaurant_user() -> Result<Json<Value>, StatusCode> {
    dotenv().ok();
    let client = client().await;
    if let Err(err) = client {
        println!("{:?}", err);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let mut session = match client.unwrap().start_session(None).await {
        Ok(session) => session,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    session.start_transaction(None).await.unwrap();

    let restaurants = get_restaurant_agg_user(&mut session).await;

    let results = Json(serde_json::to_value(&restaurants).unwrap());

    match session.commit_transaction().await {
        Ok(_) => (),
        Err(err) => {
            println!("Error committing transaction {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    Ok(results)
}

pub async fn get_sports() -> Result<Json<Value>, StatusCode> {
    dotenv().ok();
    let client = client().await;
    if let Err(err) = client {
        println!("{:?}", err);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let mut session = match client.unwrap().start_session(None).await {
        Ok(session) => session,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    session.start_transaction(None).await.unwrap();

    let sports = get_sports_agg(&mut session).await;

    let results = Json(serde_json::to_value(&sports).unwrap());

    match session.commit_transaction().await {
        Ok(_) => (),
        Err(err) => {
            println!("Error committing transaction {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    Ok(results)
}

pub async fn get_accessible_restaurants() -> Result<Json<Value>, StatusCode> {
    dotenv().ok();
    let client = client().await;
    if let Err(err) = client {
        println!("{:?}", err);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let mut session = match client.unwrap().start_session(None).await {
        Ok(session) => session,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    session.start_transaction(None).await.unwrap();

    let accessible_restaurants = get_accessible_restaurants_agg(&mut session).await;

    let results = Json(serde_json::to_value(&accessible_restaurants).unwrap());

    match session.commit_transaction().await {
        Ok(_) => (),
        Err(err) => {
            println!("Error committing transaction {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    Ok(results)
}
