use crate::auth::controller::{login, register};
use crate::auth::middleware::auth_middleware;
use crate::restaurants::contoller::{get_restaurant, new_restaurant};
use axum::middleware;
use axum::{
    routing::{get, post},
    Router,
};

// use csv::StringRecord;
// use mongodb::bson::doc;
// use mongodb::Client;
// use std::{error::Error, process};
use tokio;
use tower_http::cors::{Any, CorsLayer};

use tower::ServiceBuilder;

mod auth;
mod db;
mod restaurants;
mod server;
mod users;

#[tokio::main]
pub async fn main() {
    // run_migration().await;
    launch_server().await;
}

async fn launch_server() {
    // Load environment variables from .env file
    let app = Router::new()
        .route("/restaurant", get(get_restaurant).post(new_restaurant))
        .route_layer(middleware::from_fn(auth_middleware))
        .route("/", get(|| async { "Hello, World!" }))
        .route("/register", post(register))
        .route("/login", post(login))
        .layer(
            ServiceBuilder::new().layer(CorsLayer::new().allow_origin(Any)),
        );

    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let listen_address = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(listen_address).await.unwrap();

    std::println!("Server running on port: {port}");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
