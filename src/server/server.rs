use serde::Deserialize;

pub async fn lgbt() -> String {
    return "Hello, World!, this is the lgbt route".to_string();
}

pub async fn wifi() -> String {
    return "Hello, World!, this is the wifi route".to_string();
}

pub async fn smoker() -> String {
    return "Hello, World!, this is the smoker route".to_string();
}

#[derive(Deserialize)]
pub struct CreateRestaurantPayload {
    pub name: String,
}
