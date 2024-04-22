use crate::{auth::auth::create_jwt, users::{model::{RegisterError, UserPayload}, service::UserService}};
use axum::Json;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub async fn login(Json(payload): Json<LoginPayload>) -> Json<Value> {
    let token = create_jwt(payload.username.as_str());
    println!("{:?}", token);
    let response_token = match token {
        Ok(token) => token,
        Err(_) => panic!("Error creating JWT token"),
    };
    let response = LoginResponse {
        token: response_token,
    };
    return Json(json!(response));
}

pub async fn register(Json(payload): Json<UserPayload>) -> Result<Json<Value>, StatusCode> {
    let user = UserService::new(payload.username, payload.password).await;
    let response = match user {
        Ok(user) => json!({
            "id": user.id.to_hex(),
            "username": user.username,
        }),
        Err(e) => {
            match e {
                RegisterError::FailedToInsertUserError(_) => {
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
                RegisterError::UsernameAlreadyExistsError(_) => {
                    return Err(StatusCode::CONFLICT);
                }
            }
        }
    };
    
    return Ok(Json(response));
}

#[derive(Serialize, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}
