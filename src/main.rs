use models::restaurant::restaurant::Restaurant;
use dotenv::dotenv;
use mongodb::bson::doc;
use mongodb::{Client, Collection};
use std::{error::Error, process};
use tokio;

mod db;
mod models;
use crate::db::db::{client, file_db};
use axum::{
    routing::{get, post},
    Router,
};

async fn launch_server() {
    // Load environment variables from .env file
    dotenv().ok();
    let client: Result<Client, Box<dyn Error>> = client().await;
    if let Err(err) = client {
        println!("error running example: {}", err);
        process::exit(1);
    }
    let db_client = client.unwrap();
    let collections = db_client.database("Rustaurant").list_collection_names(None).await;
    if let Err(err) = collections {
        println!("error running example: {}", err);
        process::exit(1);
    }
    let collection_names = collections.unwrap();
    if !(collection_names.len() > 0){
        println!("Loading data into the database...");
        file_db(db_client.database("Rustaurant"))
            .await
            .expect("Failed to load data into the database.");
    }
    let collections = db_client.database("Rustaurant").list_collection_names(None).await;
    if let Err(err) = collections {
        println!("error running example: {}", err);
        process::exit(1);
    }
    let collection_names = collections.unwrap();
    for i in collection_names {
        let collection: Collection<Restaurant> =
            db_client.database("Rustaurant").collection::<Restaurant>(&*i);
        println!("Collection: {}", i);
        println!(
            "Restaurants with an outdoor: {}",
            collection
                .count_documents(doc! {"outdoor_seating": "yes"}, None)
                .await
                .expect("TODO: panic message")
        );
        println!(
            "Restaurants without an outdoor: {}",
            collection
                .count_documents(doc! {"outdoor_seating": null}, None)
                .await
                .expect("TODO: panic message")
        );
        println!("print a restaurant in this collection: ");
        let restaurant = collection.find(None, None).await.expect("TODO: panic message");
        println!("{:?}", restaurant.deserialize_current());
    }
    let new_restaurant = Restaurant {
        contact: None,
        name: Option::from("Rustaurant".to_string()),
        outdoor_seating: Some("yes".to_string()),
        indoor_seating: Some("yes".to_string()),
        ..Default::default()
    };
    let collection: Collection<Restaurant> =
        db_client.database("Rustaurant").collection::<Restaurant>("restaurant");
    collection.insert_one(new_restaurant, None).await.expect("TODO: panic message");
    let new_restaurants = collection.find(None, None).await.expect("TODO: panic message");
    println!("the new restaurant: {:?}", new_restaurants.deserialize_current());
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/lgbt", get(lgbt))
        .route("/wifi", get(wifi))
        .route("/smoker", get(smoker))
        .route("/restaurant", post(add_restaurant));

    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let listen_address = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(listen_address).await.unwrap();

    std::println!("Server running on port: {port}");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
