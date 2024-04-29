use crate::{
    api::errors::ApiError,
    auth::auth::create_jwt,
    users::{
        model::{RegisterError, UserPayload},
        service::UserService,
    },
};
use axum::Json;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub async fn login(Json(payload): Json<LoginPayload>) -> Result<Json<Value>, ApiError> {
    let username = payload.username;
    let password = payload.password;

    let mut now = chrono::Utc::now();
    let user = UserService::find(username.clone()).await;
    println!("Search user took {:?}", chrono::Utc::now() - now);

    let user = match user {
        Ok(user) => user,
        Err(_) => {
            return Err(ApiError {
                code: StatusCode::UNAUTHORIZED,
                message: "Bad username".into(),
            })
        }
    };
    now = chrono::Utc::now();
    let is_valid = UserService::verify_password(user, password).await;

    println!("Hash comparison took {:?}", chrono::Utc::now() - now);
    if !is_valid {
        return Err(ApiError {
            code: StatusCode::UNAUTHORIZED,
            message: "Bad password".into(),
        });
    }

    now = chrono::Utc::now();
    let token = create_jwt(&username.as_str());
    println!("Token generation took {:?}", chrono::Utc::now() - now);
    println!("{:?}", token);
    let response_token = match token {
        Ok(token) => token,
        Err(_) => panic!("Error creating JWT token"),
    };
    let response = LoginResponse {
        token: response_token,
    };
    Ok(Json(json!(response)))
}

pub async fn register(Json(payload): Json<UserPayload>) -> Result<Json<Value>, StatusCode> {
    let user = UserService::new(payload.username, payload.password).await;
    let response = match user {
        Ok(user) => json!({
            "id": user._id.to_hex(),
            "username": user.username,
        }),
        Err(e) => match e {
            RegisterError::FailedToInsertUserError(_) => {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
            RegisterError::UsernameAlreadyExistsError(_) => {
                return Err(StatusCode::CONFLICT);
            }
        },
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
