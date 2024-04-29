use bson::{doc, Bson, Document};
use mongodb::{ClientSession, Collection};

use super::model::Restaurant;

pub async fn get_restaurant_agg_user(session: &mut ClientSession) -> Vec<Document> {
    let user_restaurant_pipeline = vec![
        doc! {
            "$match": doc! {
                "osm_user": doc! {
                  "$exists": true,
                  "$ne": "",
                },
            }
        },
        doc! {
            "$lookup": doc! {
                "from": "users",
                "localField": "osm_user",
                "foreignField": "username",
                "as": "creator",
            }
        },
        doc! {
            "$sort": doc!{
                "osm_user": 1,
            }
        },
        doc! {
            "$project": doc! {
                "creator.password": 0
            }
        },
    ];
    agregate(session, "restaurant".to_string(), user_restaurant_pipeline).await
}

async fn agregate(
    session: &mut ClientSession,
    collection: String,
    pipeline: Vec<Document>,
) -> Vec<Document> {
    let db_client = session.client();

    let collection: Collection<Restaurant> =
        db_client.database("Rustaurant").collection(&collection);

    let mut user_restaurant_cursor = collection.aggregate(pipeline, None).await.unwrap();

    let mut user_restaurant = vec![];

    loop {
        let result = user_restaurant_cursor.advance().await;
        match result {
            Ok(doc) => {
                if doc {
                    user_restaurant.push(user_restaurant_cursor.deserialize_current().unwrap());
                } else {
                    break;
                }
            }
            Err(err) => {
                println!("Error getting user restaurant {:?}", err);
                break;
            }
        }
    }
    return user_restaurant;
}

pub async fn get_sports_agg(session: &mut ClientSession) -> Vec<Document> {
    let sport_pipeline = vec![
        doc! {
            "$match": doc! {
                "sport": doc! {
                    "$exists": true
                }
            }
        },
        doc! {
            "$group": doc! {
                "_id": Bson::Null,
                "sports": doc! {
                    "$addToSet": "$sport"
                }
            }
        },
        doc! {
            "$project": doc! {
                "_id": 0,
                "sports": 1
            }
        },
    ];

    agregate(session, "pub".to_string(), sport_pipeline).await
}

pub async fn get_accessible_restaurants_agg(session: &mut ClientSession) -> Vec<Document> {
    let accessible_restaurants_pipeline = vec![
        doc! {
            "$lookup": doc! {
                "from": "restaurant",
                "let": doc! {
                    "sitted_width": "$largeur_assise"
                },
                "pipeline": [
                    doc! {
                        "$match": doc! {
                            "entrance": doc! {
                                "$ne": Bson::Null
                            }
                        }
                    },
                    doc! {
                        "$addFields": doc! {
                            "door_width": doc! {
                                "$toDouble": "$entrance.width"
                            }
                        }
                    },
                    doc! {
                        "$match": doc! {
                            "$expr": doc! {
                                "$lt": [
                                    "$$sitted_width",
                                    "$door_width"
                                ]
                            }
                        }
                    }
                ],
                "as": "restaurants"
            }
        }
    ];
    agregate(session, "restaurant".to_string(),accessible_restaurants_pipeline).await
}