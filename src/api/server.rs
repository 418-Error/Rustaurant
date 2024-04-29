use mongodb::Client;

pub struct AppState {
    pub db: Client,
}