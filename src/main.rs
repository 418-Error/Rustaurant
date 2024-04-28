use crate::auth::controller::{login, register};
use crate::db::db::create_indexes;
use dotenv::dotenv;
use crate::auth::middleware::auth_middleware;
use crate::restaurants::contoller::{delete_restaurant, get_restaurant, new_restaurant};
use axum::middleware;
use axum::{
    routing::{get, post},
    Router,
};

use db::db::{client, file_db, run_migration};
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
    // dotenv().ok();
    // let client = client().await;
    // if let Err(err) = client {
    //     println!("error launching client : {}", err);
    //     std::process::exit(1);
    // }

    // let db_client = client.unwrap().database("Rustaurant");
    // match file_db(db_client).await {
    //     Ok(_) => {
    //         create_indexes().await;
    //         println!("Database connected")
    //     },
    //     Err(err) => {
    //         println!("error connecting to database : {}", err);
    //         std::process::exit(1);
    //     }
    // }

    launch_server().await;
}

async fn launch_server() {
    // Load environment variables from .env file
    let app = Router::new()
        .route("/restaurant", post(new_restaurant).get(get_restaurant).delete(delete_restaurant))
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
