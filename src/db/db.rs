use dotenv::dotenv;
use mongodb::bson::doc;
use mongodb::options::{ClientOptions, ResolverConfig};
use mongodb::{Client, Collection, Database, IndexModel};
use std::collections::HashMap;
use std::{error::Error, process};

use crate::restaurants::model::Restaurant;

pub async fn client() -> Result<Client, Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
    dotenv().ok();
    let client_uri =
        std::env::var("MONGO_URI").expect("You must set the MONGO_URI environment var!");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    Ok(client)
}

pub async fn create_indexes() {
    dotenv().ok();
    let collection_indexes = HashMap::from([
        ("bar", vec!["name"]),
        ("cafe", vec!["name"]),
        ("fast_food", vec!["name"]),
        ("ice_cream", vec!["name"]),
        ("others", vec!["name"]),
        ("pub", vec!["name"]),
        ("restaurant", vec!["name"]),
        ("users", vec!["username"]),
    ]);

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
    for i in collection_names {
        let collection: Collection<Restaurant> =
            db_client.database("Rustaurant").collection::<Restaurant>(&*i);
        let indexes = collection_indexes.get(&*i);
        for index in indexes.unwrap() {
            let index_model = IndexModel::builder().keys(doc! {*index: 1}).build();
            collection
                .create_index(index_model, None)
                .await
                .expect("TODO: panic message");
        }
    }
}

pub async fn run_migration() {
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
}

pub(crate) async fn file_db(db: Database) -> Result<(), Box<dyn Error>> {
    let body = reqwest::get("https://data.montpellier3m.fr/sites/default/files/ressources/OSM_Metropole_restauration_bar.json").await;
    let response = match body {
        Ok(response) => response,
        Err(err) => {
            println!("Error fetching data: {}", err);
            process::exit(1);
        }
    };
    let json: serde_json::Value = response.json().await?;
    for i in json.get("features").unwrap().as_array().unwrap() {
        let mut doc = doc! {};
        let mut previous_key = "".to_string();
        let mut concat: Vec<String> = Vec::new();
        let tags = i
            .get("properties")
            .unwrap()
            .get("tags")
            .unwrap()
            .as_str()
            .unwrap();
        for i in tags.rsplit(", ") {
            let elem: Vec<&str> = i.split("=>").collect();
            let is_good = match elem.len() {
                2 => true,
                _ => false,
            };
            if is_good {
                if concat.len() > 0 {
                    let split: Vec<&str> = previous_key.split(":").collect();
                    if split.len() == 2 {
                        let last = doc.get(split[0].replace("\"", ""));
                        if last.is_some() {
                            let ancient = last.unwrap();
                            let new = ancient.as_document();
                            if !new.is_none() {
                                let mut new = new.unwrap().clone();
                                new.insert(split[1].replace("\"", ""), concat);
                                doc.insert(split[0].replace("\"", ""), new);
                                concat = Vec::new();
                            }
                        } else {
                            doc.insert(
                                split[0].replace("\"", ""),
                                doc! {split[1].replace("\"",""): concat},
                            );
                            concat = Vec::new();
                        }
                    } else {
                        doc.insert(previous_key, concat);
                        concat = Vec::new();
                    }
                }
                let split: Vec<&str> = elem[0].split(":").collect();
                if split.len() == 2 {
                    let last = doc.get(split[0].replace("\"", ""));
                    if last.is_some() {
                        let ancient = last.unwrap();
                        let new = ancient.as_document();
                        if !new.is_none() {
                            let mut new = new.unwrap().clone();
                            new.insert(split[1].replace("\"", ""), elem[1].replace("\"", ""));
                            doc.insert(split[0].replace("\"", ""), new);
                            previous_key = elem[0].replace("\"", "");
                            continue;
                        }
                    }
                    doc.insert(
                        split[0].replace("\"", ""),
                        doc! {split[1].replace("\"",""): elem[1].replace("\"","")},
                    );
                    previous_key = elem[0].replace("\"", "");
                    continue;
                }
                doc.insert(elem[0].replace("\"", ""), elem[1].replace("\"", ""));
                previous_key = elem[0].replace("\"", "");
            } else {
                let inter = elem[0].replace("\"", "");
                concat.push(inter);
            }
        }
        let amenity = doc.get("amenity");
        let mut collection_name = "others";
        if !amenity.is_none() {
            let amenity = amenity.unwrap().as_str();
            if !amenity.is_none() {
                collection_name = amenity.unwrap();
            }
        }
        db.collection(collection_name)
            .insert_one(doc.clone(), None)
            .await?;
    }
    create_indexes().await;
    Ok(())
}

pub async fn add_restaurant(db: Database, restaurant: Restaurant) -> Result<(), Box<dyn Error>> {
    let mut collection = "others";
    if !restaurant.amenity.is_none() {
        collection = restaurant.amenity.as_ref().unwrap();
    }
    let db_collection = db.collection(collection);
    db_collection.insert_one(restaurant, None).await?;
    Ok(())
}

pub async fn most_complexe() -> Result<mongodb::bson::Document, Box<dyn Error>> {
    let body = reqwest::get("https://data.montpellier3m.fr/sites/default/files/ressources/OSM_Metropole_restauration_bar.json").await;
    let response = match body {
        Ok(response) => response,
        Err(err) => {
            println!("Error fetching data: {}", err);
            process::exit(1);
        }
    };
    let mut doc = doc! {};
    let json: serde_json::Value = response.json().await?;
    for i in json.get("features").unwrap().as_array().unwrap() {
        let mut previous_key = "".to_string();
        let mut concat: Vec<String> = Vec::new();
        let tags = i
            .get("properties")
            .unwrap()
            .get("tags")
            .unwrap()
            .as_str()
            .unwrap();
        for i in tags.rsplit(", ") {
            let elem: Vec<&str> = i.split("=>").collect();
            let is_good = match elem.len() {
                2 => true,
                _ => false,
            };
            if is_good {
                if concat.len() > 0 {
                    let split: Vec<&str> = previous_key.split(":").collect();
                    if split.len() == 2 {
                        let last = doc.get(split[0].replace("\"", ""));
                        if last.is_some() {
                            let ancient = last.unwrap();
                            let new = ancient.as_document();
                            if !new.is_none() {
                                let mut new = new.unwrap().clone();
                                new.insert(split[1].replace("\"", ""), concat);
                                doc.insert(split[0].replace("\"", ""), new);
                                concat = Vec::new();
                            }
                        } else {
                            doc.insert(
                                split[0].replace("\"", ""),
                                doc! {split[1].replace("\"",""): concat},
                            );
                            concat = Vec::new();
                        }
                    } else {
                        doc.insert(previous_key, concat);
                        concat = Vec::new();
                    }
                }
                let split: Vec<&str> = elem[0].split(":").collect();
                if split.len() == 2 {
                    let last = doc.get(split[0].replace("\"", ""));
                    if last.is_some() {
                        let ancient = last.unwrap();
                        let new = ancient.as_document();
                        if !new.is_none() {
                            let mut new = new.unwrap().clone();
                            new.insert(split[1].replace("\"", ""), elem[1].replace("\"", ""));
                            doc.insert(split[0].replace("\"", ""), new);
                            previous_key = elem[0].replace("\"", "");
                            continue;
                        }
                    }
                    doc.insert(
                        split[0].replace("\"", ""),
                        doc! {split[1].replace("\"",""): elem[1].replace("\"","")},
                    );
                    previous_key = elem[0].replace("\"", "");
                    continue;
                }
                doc.insert(elem[0].replace("\"", ""), elem[1].replace("\"", ""));
                previous_key = elem[0].replace("\"", "");
            } else {
                let inter = elem[0].replace("\"", "");
                concat.push(inter);
            }
        }
    }
    Ok(doc)
}
