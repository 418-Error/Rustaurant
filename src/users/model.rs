use std::error::Error;

use serde_json::error;
use thiserror::Error as ThisError;

use bson::{doc, oid::ObjectId};
use dotenv::dotenv;
use mongodb::Client;
use serde::{Deserialize, Serialize};

use crate::db::db::client;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug, ThisError)]
pub enum RegisterError {
    #[error("Failed to insert user")]
    FailedToInsertUserError(String),

    #[error("Username already exists")]
    UsernameAlreadyExistsError(String),
}

#[derive(Debug, ThisError)]
pub enum FindError {
    #[error("Failed to find user")]
    FailedToFindUserError(String),
}


impl User {
    pub async fn new(username: String, password: String) -> Result<User, RegisterError> {
        dotenv().ok();
        let client: Result<Client, Box<dyn Error>> = client().await;
        if let Err(err) = client {
            println!("error launching client : {}", err);
            std::process::exit(1);
        }
        let db_client = client.unwrap();
        let collection: mongodb::Collection<UserPayload> =
            db_client.database("Rustaurant").collection("users");

        let filter= doc!{"username": &username};

        let distinct_usernames_result = collection
            .distinct(&"username", filter, None)
            .await
            .expect("Failed to count the number of users in the database.");

        let distinct_usernames = distinct_usernames_result.len();

        if distinct_usernames > 0 {
            println!("Username already exists {}", distinct_usernames);
            return Err(RegisterError::UsernameAlreadyExistsError(
                "Username already exists".to_string(),
            ));
        }

        let clone_username = username.clone();
        let clone_password = password.clone();

        let user = UserPayload {
            username: clone_username,
            password: clone_password,
        };



        let insert_result = match collection
            .insert_one(user, None)
            .await
        {
            Ok(insert_result) => insert_result,
            Err(err) => {
                return Err(RegisterError::FailedToInsertUserError(
                    format!("Failed to insert user: {}", err),
                ));
            }
        };


        Ok(User {
            _id: insert_result
                .inserted_id
                .as_object_id()
                .expect("Failed to insert user into the database."),
            username,
            password,
        })
    }

    pub async fn find_by_username(username: String) -> Result<User, FindError> {
        dotenv().ok();
        let client: Result<Client, Box<dyn Error>> = client().await;
        if let Err(err) = client {
            println!("error launching client : {}", err);
            std::process::exit(1);
        }
        let db_client = client.unwrap();
        let collection: mongodb::Collection<User> =
            db_client.database("Rustaurant").collection("users");


        let filter= doc!{"username": username};

        println!("{:?}", filter);

        let user = match collection
            .find_one(filter, None)
            .await {
            Ok(user) => user,
            Err(err) => {
                println!("Failed to find user: {}", err);
                return Err(FindError::FailedToFindUserError(
                    format!("Failed to find user: {}", err),
                ));
            }
        };

        println!("{:?}", user);

            

        match user {
            Some(user) => Ok(user),
            None => Err(FindError::FailedToFindUserError(
                "Failed to find user in the database.".to_string(),
            )),
        }
    }
}
