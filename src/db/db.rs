use mongodb::{Client, Database};
use std::{error::Error};
use dotenv::dotenv;
use mongodb::options::{ClientOptions, ResolverConfig};
use mongodb::bson::doc;

pub(crate) async fn client() -> Result<Client, Box<dyn Error>>{
    // Load the MongoDB connection string from an environment variable:
    dotenv().ok();
    let client_uri = std::env::var("MONGO_URI")
        .expect("You must set the MONGO_URI environment var!");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    Ok(client)
}

pub(crate) async fn file_db(db: Database) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("OSM_Metropole_restauration_bar.csv")?;
    for result in rdr.records() {
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
        db.collection("restaurants").insert_one(doc.clone(), None).await?;
    }
    Ok(())
}