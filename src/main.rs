use crate::auth::controller::{login, register};
use crate::auth::middleware::auth_middleware;
use crate::restaurants::controller::{delete_restaurant, get_accessible_restaurants, get_restaurant, get_restaurant_user, get_sports, new_restaurant, update_restaurant};
use crate::routes::routes::{graphql_handler, graphql_playground, QueryRoot};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{middleware, Extension};
use axum::{
    routing::{get, post},
    Router,
};

use tokio;
use tower_http::cors::{Any, CorsLayer};

use tower::ServiceBuilder;
use tracing::info;

mod auth;
mod db;
mod restaurants;
mod server;
mod users;
mod routes;
mod api;

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();
    // run_migration().await;
    launch_server().await;
}

//TODO: pool de client mongo db
async fn launch_server() {

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
            ServiceBuilder::new().layer(CorsLayer::very_permissive()),
        );


    let app = Router::new().merge(protected_router).merge(public_router);
       
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let listen_address = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(listen_address).await.unwrap();

    info!("Server running on port: {port}");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
