use bson::{doc, oid::ObjectId, to_document, Document};
use mongodb::{error::Error as MongoError, results::{DeleteResult, InsertOneResult, UpdateResult}, ClientSession};
use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Restaurant {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_id")]
    pub id: Option<ObjectId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addr: Option<Addr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "outdoor_seating")]
    pub outdoor_seating: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "osm_timestamp")]
    pub osm_timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "osm_changeset")]
    pub osm_changeset: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "osm_version")]
    pub osm_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "osm_user")]
    pub osm_user: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smoking: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "osm_uid")]
    pub osm_uid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amenity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sport: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wheelchair: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brewery: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "entrance:step:height")]
    pub entrance_step_height: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entrance: Option<Entrance>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cuisine: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bar: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "internet_access")]
    pub internet_access: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub survey: Option<Survey>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "indoor_seating")]
    pub indoor_seating: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "opening_hours")]
    pub opening_hours: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "")]
    pub field: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub toilets: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "air_conditioning")]
    pub air_conditioning: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "official_name")]
    pub official_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub microbrewery: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dog: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "old_name")]
    pub old_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "check_date")]
    pub check_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub takeaway: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "drive_through")]
    pub drive_through: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wikidata: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stars: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diet: Option<Diet>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access: Option<Access>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment: Option<Payment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "min_age")]
    pub min_age: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "alt_name")]
    pub alt_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "short_name")]
    pub short_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disused: Option<Disused>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ref:FR:SIRET")]
    pub ref_fr_siret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "alt_name_1")]
    pub alt_name_1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "start_date")]
    pub start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub beverage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub food: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terrace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub newsagent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tobacco: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "education_profile")]
    pub education_profile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub school: Option<School>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "drive_in")]
    pub drive_in: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alcohol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facebook: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub craft: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shop: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "full_name")]
    pub full_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drink: Option<Drink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mapillary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "phone_1")]
    pub phone_1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "service_times")]
    pub service_times: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restaurant: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lgbtq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mhs: Option<Mhs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heritage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ref")]
    pub ref_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batiment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pub")]
    pub pub_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "street_vendor")]
    pub street_vendor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highchair: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "self_service")]
    pub self_service: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "way_area")]
    pub way_area: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub building: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wall: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indoor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reservation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fast_food")]
    pub fast_food: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facebook: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub housenumber: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postcode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instagram: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tripadvisor: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Addr {
    pub housenumber: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postcode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub housename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entrance {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "step_count")]
    pub step_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Survey {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vegetarian: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vegan: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lactose_free")]
    pub lactose_free: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meat: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mediterranean: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub halal: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "gluten_free")]
    pub gluten_free: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub healthy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "non-vegetarian")]
    pub non_vegetarian: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kosher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Access {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub covid19: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "credit_cards")]
    pub credit_cards: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "debit_cards")]
    pub debit_cards: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "meal_voucher")]
    pub meal_voucher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "american_express")]
    pub american_express: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mastercard: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contactless: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "titre_restaurant")]
    pub titre_restaurant: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cheques_vacances")]
    pub cheques_vacances: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "apple_pay")]
    pub apple_pay: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coins: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mastercard_contactless")]
    pub mastercard_contactless: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "visa_electron")]
    pub visa_electron: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "visa_debit")]
    pub visa_debit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maestro: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cards: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Disused {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amenity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct School {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Drink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wine: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub beer: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mhs {
    #[serde(rename = "inscription_date")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inscription_date: Option<String>,
}

impl Restaurant {
    pub async fn save(mut self, username: String, session: &mut ClientSession) -> Result<InsertOneResult, MongoError> {
        self.osm_timestamp = Some(chrono::Utc::now().to_string());
        self.osm_changeset = Some("0".to_string());
        self.osm_version = Some("0".to_string());
        self.osm_user = Some(username);

        let collection = self.amenity.clone().map_or("others".to_string(), |amenity| amenity);

        let db_client = session.client();
        let collection: mongodb::Collection<Restaurant> = db_client
            .database("Rustaurant")
            .collection(&collection);

        let insert_result = collection.insert_one(self, None).await;
        insert_result
    }

    pub async fn find_by_kind(name: &str, kind: String, session: &mut ClientSession) -> Result<Vec<Restaurant>, MongoError> {
        let filter = doc! { "name": name };
        Restaurant::find(filter, kind, session).await
    }

    pub async fn find_by_name(name: String, session: &mut ClientSession) -> Result<Vec<Restaurant>, MongoError> {
        let collections = vec![
            "restaurant".to_string(),
            "bar".to_string(),
            "pub".to_string(),
            "cafe".to_string(),
        ];

        let mut restaurants = Vec::new();

        for collection in collections.iter() {
            let result = Restaurant::find_by_kind(&name, collection.clone(), session).await;
            match result {
                Ok(mut restaurant) => {
                    restaurants.append(&mut restaurant);
                }
                Err(err) => return Err(err),
            }
        }
        Ok(restaurants)
    }

    pub async fn find(filter: Document, collection: String, session: &mut ClientSession) -> Result<Vec<Restaurant>, MongoError> {
        let db_client = session.client();
        let collection: mongodb::Collection<Restaurant> = db_client
            .database("Rustaurant")
            .collection(&collection);
        let query_result = collection.find(filter, None).await;
        let mut restaurants = Vec::new();
        match query_result {
            Ok(mut cursor) => {
                while cursor.advance().await? {
                    let restaurant = cursor.deserialize_current();
                    match restaurant {
                        Ok(restaurant) => restaurants.push(restaurant),
                        Err(err) => {
                            println!("Error getting restaurant: {}", err);
                        }
                    }
                }
            }
            Err(err) => {
                println!("Error getting restaurant: {}", err);
            }
        }
        Ok(restaurants)
    }

    pub async fn delete(&self, session: &mut ClientSession) -> Result<DeleteResult, MongoError> {
        if self.name.is_none() {
            return Err(MongoError::from(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "name is required",
            )));
        }
        let db_client = session.client();
        let name = self.name.clone().map_or("".to_string(), |name| name);
        let filter = doc! { "name": self.name.clone().unwrap() };
        let collection: mongodb::Collection<Restaurant> = db_client
            .database("Rustaurant")
            .collection(find_restaurant_collection(filter, session).await.as_str());
        println!("{:?}", name);
        let filter = doc! { "name": name };
        let delete_result = collection.delete_many(filter, None).await;
        delete_result
    }

    pub async fn update(mut self, session: &mut ClientSession) -> Result<UpdateResult, MongoError> {
        let id = self.id.clone().unwrap();
        let filter_id = doc! { "_id": id};
        println!("{:?}", filter_id);

        let collection_name = find_restaurant_collection(filter_id.clone(), session).await;

        let db_client = session.client();

        let restaurant = Restaurant::find(filter_id.clone(), collection_name.clone(), session).await;

        println!("{:?}", restaurant);

        let restaurant = match restaurant {
            Ok(restaurants) => {
                if restaurants.len() == 0 {
                    return Err(MongoError::from(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Restaurant not found",
                    )));
                }
                restaurants.get(0).unwrap().clone()
            }
            Err(err) => return Err(err),
        };

        self.osm_timestamp = Some(chrono::Utc::now().to_string());
        self.osm_version = Some(restaurant.osm_version.clone().unwrap());
        let collection: mongodb::Collection<Restaurant> = db_client
            .database("Rustaurant")
            .collection(&collection_name);
        let to_doc = to_document(&self).unwrap();
        let update = doc! { "$set": to_doc };
        let update_result = collection.update_one(filter_id, update, None).await;
        update_result
    }
}

async fn find_restaurant_collection(filter: Document, session: &mut ClientSession) -> String {
    let collection_names = get_collections(session).await;
    let db_client = session.client();
    let db = db_client.database("Rustaurant");
    for collection_name in collection_names.iter() {
        let collection: mongodb::Collection<Restaurant> = db.collection(collection_name);
        let query_result = collection.find_one(filter.clone(), None).await;
        match query_result {
            Ok(result) => match result {
                Some(_) => {
                    if collection_name.to_string() == "wheelingChairs" || collection_name.to_string() == "users" {
                        return "others".to_string();
                    }
                    return collection_name.to_string();
                },
                None => continue,
            },
            Err(_) => continue,
        }
    }
    "others".to_string()
}

async fn get_collections(session: &mut ClientSession) -> Vec<String> {
    let db_client = session.client();
    let db = db_client.database("Rustaurant");
    let collections = db.list_collection_names(None).await.unwrap();
    collections
}