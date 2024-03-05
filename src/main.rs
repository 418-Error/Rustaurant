// use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::{error::Error, process};
use bson::RawArray;
use csv::StringRecord;
// use csv::StringRecord;
use mongodb::bson::doc;
// use dotenv::dotenv;
// use tokio;

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

fn read_csv() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path("OSM_Metropole_restauration_bar.csv")?;
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here..
        let mut doc = doc!{};
        let mut previous_key = "".to_string();
        let mut concat: Vec<String> = Vec::new();
        let record = result?;
        for i in record[18].rsplit(", "){
            let elem: Vec<&str>= i.split("=>").collect();
            let is_good = match elem.len() {
                2 => true,
                _ => false,
            };
            if is_good {
                if concat.len() > 0 {
                    let split: Vec<&str> = previous_key.split(":").collect();
                    if split.len() == 2 {
                        let last = doc.get(split[0].replace("\"",""));
                        if last.is_some() {
                            let ancient = last.unwrap();
                            let new = ancient.as_document();
                            if !new.is_none() {
                                let mut new = new.unwrap().clone();
                                new.insert(split[1].replace("\"",""), concat);
                                doc.insert(split[0].replace("\"",""), new);
                                concat = Vec::new();
                                }
                        }
                        else{
                            doc.insert(split[0].replace("\"",""), doc!{split[1].replace("\"",""): concat});
                            concat = Vec::new();
                        }
                    }
                    else {
                        doc.insert(previous_key, concat);
                        concat = Vec::new();
                    }
                }
                let split: Vec<&str> = elem[0].split(":").collect();
                if split.len() == 2 {
                    let last = doc.get(split[0].replace("\"",""));
                    if last.is_some() {
                        let ancient = last.unwrap();
                        let new = ancient.as_document();
                        if !new.is_none() {
                            let mut new = new.unwrap().clone();
                            new.insert(split[1].replace("\"",""), elem[1].replace("\"",""));
                            doc.insert(split[0].replace("\"",""), new);
                            previous_key = elem[0].replace("\"", "");
                            continue;
                        }
                    }
                    doc.insert(split[0].replace("\"",""), doc!{split[1].replace("\"",""): elem[1].replace("\"","")});
                    previous_key = elem[0].replace("\"", "");
                    continue;
                }
                doc.insert(elem[0].replace("\"",""), elem[1].replace("\"",""));
                previous_key = elem[0].replace("\"", "");
            }
            else {
                let inter = elem[0].replace("\"", "");
                concat.push(inter);
            }
        }
        println!("{:?}", doc);
    }
    Ok(())
}

// #[tokio::main]
// async fn mongo_connection() -> Result<(), Box<dyn Error>> {
//     // Load the MongoDB connection string from an environment variable:
//     dotenv().ok();
//     let client_uri = std::env::var("MONGO_URI")
//         .expect("You must set the MONGO_URI environment var!");
//
//     // A Client is needed to connect to MongoDB:
//     // An extra line of code to work around a DNS issue on Windows:
//     let options =
//         ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
//             .await?;
//     let client = Client::with_options(options)?;
//
//     // Print the databases in our MongoDB cluster:
//     println!("Databases:");
//     for name in client.list_database_names(None, None).await? {
//         println!("- {}", name);
//     }
//
//     Ok(())
// }

fn main() {
    if let Err(err) = read_csv() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}