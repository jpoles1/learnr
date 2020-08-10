use mongodb::{Client, Collection};

pub async fn connect(collection: String) -> Collection {
    let mongo_client = Client::with_uri_str("mongodb://127.0.0.1:27017").await.unwrap();
    let db = mongo_client.database("learnr");
    let entries = db.collection(&collection);
    return entries;
}