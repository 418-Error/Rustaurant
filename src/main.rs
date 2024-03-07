use crate::db::db::{client, file_db};
use csv::StringRecord;
use dotenv::dotenv;
use mongodb::bson::doc;
use mongodb::Client;
use std::{error::Error, process};
use tokio;

pub mod db;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let client: Result<Client, Box<dyn Error>> = client().await;
    if let Err(err) = client {
        println!("error running example: {}", err);
        process::exit(1);
    }
    let db_client = client.unwrap();
    let collection: mongodb::Collection<StringRecord> =
        db_client.database("Rustaurant").collection("restaurants");
    let db = db_client.database("Rustaurant");
    if collection
        .count_documents(doc! {}, None)
        .await
        .expect("TODO: panic message")
        < 1
    {
        print!("Loading data into the database...");
        file_db(db)
            .await
            .expect("Failed to load data into the database.");
        println!(
            " {}",
            collection
                .count_documents(doc! {}, None)
                .await
                .expect("TODO: panic message")
        );
    }
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
}
