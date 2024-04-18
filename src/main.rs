use crate::db::db::{client, file_db, most_complexe};
use dotenv::dotenv;
use mongodb::bson::doc;
use mongodb::{Client, Collection};
use std::{error::Error, process};
use serde::{Deserialize, Serialize};
use tokio;

pub mod db;

// #[derive(Clone, Debug, Deserialize, Serialize)]
// struct Record {
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     addr_street: Option<String>,
//     #[serde(skip_s erializing_if = "Option::is_none")]
//     addr_housenumber: Option<u64>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     name: Option<String>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     brand: Option<String>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     operator: Option<String>,
// }


#[tokio::main]
async fn main() {
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
        let collection: Collection<Record> =
            db_client.database("Rustaurant").collection::<Record>(&*i);
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
}
