use crate::db::db::{client, file_db };
use mongodb::{Client};
use std::{error::Error, process};
use csv::StringRecord;
use mongodb::bson::doc;
use tokio;

pub mod db;

// #[derive(Debug, Deserialize)]
// struct OpeningHour {
//     opening_hour: Option<Time>,
//     closing_hour: Option<Time>,
// }

// #[derive(Debug, Deserialize)]
// struct OpeningHours {
//     monday: Option<OpeningHour>,
//     tuesday: Option<OpeningHour>,
//     wednesday: Option<OpeningHour>,
//     thursday: Option<OpeningHour>,
//     friday: Option<OpeningHour>,
//     saturday: Option<OpeningHour>,
//     sunday: Option<OpeningHour>,
// }

// #[derive(Debug, Deserialize)]
// struct Record {
//     osm_id: u64,
//     addr_street: Option<String>,
//     addr_housenumber: Option<u64>,
//     amenity: String,
//     name: Option<String>,
//     brand: Option<String>,
//     operator: Option<String>,
//     ref: Option<u64>,
//     wheelchair: bool,
//     internet_access: bool,
//     opening_hours: Option<OpeningHours>,
//     drive_through: Option<bool>,
//     building: Option<String>,
//     cuisine: Option<[String]>,
//     capacity: Option<u64>,
//     tourism: Option<String>,
//     osm_user: Option<String>,
//     osm_timestamp: Option<String>,
//     tags,
// }

#[tokio::main]
async fn main() {
    let client: Result<Client, Box<dyn Error>> = client().await;
    if let Err(err) = client {
        println!("error running example: {}", err);
        process::exit(1);
    }
    let db_client = client.unwrap();
    let collection: mongodb::Collection<StringRecord> = db_client.database("Rustaurant").collection("restaurants");
    let db = db_client.database("Rustaurant");
    if collection.count_documents(doc! {}, None).await.expect("TODO: panic message") < 1 {
        print!("Loading data into the database...");
        file_db(db).await.expect("Failed to load data into the database.");
        println!(" {}", collection.count_documents(doc! {}, None).await.expect("TODO: panic message"));
    }
    println!("Restaurants with an outdoor: {}", collection.count_documents(doc! {"outdoor_seating": "yes"}, None).await.expect("TODO: panic message"));
    println!("Restaurants without an outdoor: {}", collection.count_documents(doc! {"outdoor_seating": null}, None).await.expect("TODO: panic message"));
}