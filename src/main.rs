// use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::{error::Error, process};
use mongodb::bson::doc;
// use dotenv::dotenv;
// use tokio;

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
//     opening_hours: Option<tim>,
//     drive_through,
//     building,
//     cuisine,
//     capacity,
//     tourism,
//     osm_user,
//     osm_timestamp,
//     tags,
// }

fn read_csv() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path("OSM_Metropole_restauration_bar.csv")?;
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here..
        let mut doc = doc!{};
        let record = result?;
        for (i) in record[18].rsplit(", "){
            let elem: Vec<&str>= i.split("=>").collect();
            doc.insert(elem[0], elem[1]);
        }
        println!("{:?}", doc.get("osm_id"));
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