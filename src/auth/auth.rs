use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct JWTTokenCreationError;

impl std::fmt::Display for JWTTokenCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error creating JWT token")
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_jwt(username: &str) -> Result<String, JWTTokenCreationError> {
    let jwt_secret = std::env::var("JWT_KEY").unwrap_or("none".to_string());
    let expiration = Utc::now()
        .checked_add_signed(chrono::TimeDelta::try_minutes(60*5).expect("32"))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|_| JWTTokenCreationError)
}

pub fn verify_token(token: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let jwt_secret = std::env::var("JWT_KEY").unwrap_or("none".to_string());
    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &jsonwebtoken::Validation::new(Algorithm::HS512),
    )?;
    Ok(token_data.claims.sub)
}
