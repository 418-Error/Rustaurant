use std::sync::Arc;

use crate::api::server::AppState;
use crate::auth::controller::{login, register};
use crate::auth::middleware::auth_middleware;
use crate::db::db::client;
use crate::restaurants::controller::{delete_restaurant, get_accessible_restaurants, get_restaurant, get_restaurant_user, get_sports, new_restaurant, update_restaurant};
use axum::middleware;
use axum::{
    routing::{get, post},
    Router,
};

use tokio;
use tower_http::cors::{Any, CorsLayer};

use tower::ServiceBuilder;
use tracing::info;
use crate::db::db::run_migration;

mod auth;
mod db;
mod restaurants;
mod api;
mod users;

#[tokio::main]
pub async fn main() {
    run_migration().await;
    launch_server().await;
}

async fn launch_server() {
    tracing_subscriber::fmt::init();
    let client = client().await;

    if let Err(err) = client {
        panic!("Error connecting to database: {:?}", err);
    }

    let app = AppState {
        db: client.unwrap()
    };

    let shared_state = Arc::new(app);

    let protected_router = Router::new()
        .route("/restaurant", 
            post(new_restaurant)
            .get(get_restaurant)
            .delete(delete_restaurant)
            .patch(update_restaurant))
        .route("/restaurant/creators", get(get_restaurant_user))
        .route("/restaurant/sports", get(get_sports))
        .route("/restaurant/accessibility", get(get_accessible_restaurants))
        .route_layer(middleware::from_fn(auth_middleware));

    let public_router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/register", post(register))
        .route("/login", post(login))
        .layer(
            ServiceBuilder::new().layer(CorsLayer::new().allow_origin(Any)),
        );


    let app = Router::new()
        .merge(protected_router)
        .merge(public_router)
        .with_state(shared_state);
       
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let listen_address = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(listen_address).await.unwrap();

    info!("Server running on port: {port}");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
