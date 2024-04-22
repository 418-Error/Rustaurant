use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use super::model::User;

pub struct UserService {}

impl UserService {
    pub async fn new(
        username: String,
        password: String,
    ) -> Result<User, super::model::RegisterError> {
        let b_password = password.as_bytes();
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let hashed_password = argon2.hash_password(b_password, &salt).unwrap().to_string();

        User::new(username, hashed_password).await
    }
}
