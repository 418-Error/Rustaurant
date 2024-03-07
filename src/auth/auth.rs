use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

enum Error {
    JWTTokenCreationError,
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    username: String,
    sub: String,
    exp: usize,
}

pub fn create_jwt(username: &str, email: &str) -> () {
    let jwt_secret = std::env::var("JWT_KEY").unwrap_or("none".to_string());
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: email.to_owned(),
        username: username.to_owned(),
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret("b{jwt_secret}"))
        .map_err(|_| Error::JWTTokenCreationError);
}
