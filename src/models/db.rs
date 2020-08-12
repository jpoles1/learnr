use mongodb::{Client, Collection};

pub async fn connect(collection: String) -> Result<Collection, String> {
    let client_res = Client::with_uri_str("mongodb://127.0.0.1:27017/?socketTimeoutMS=1000").await;
    match client_res {
        Ok(mongo_client) => {
            let db = mongo_client.database("learnr");
            let entries = db.collection(&collection);
            return Ok(entries);
        },
        Err(mongo_err) => return Err(format!("{}", mongo_err))
    }

}