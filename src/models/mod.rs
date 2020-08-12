use mongodb::bson::{doc, oid::ObjectId, Bson::Document, to_bson};
use restson::RestPath;
use async_trait::async_trait;

mod db;

#[async_trait]
pub trait MongoSchema<T> {
    async fn insert(&self) -> Result<(), String>;
    async fn update(&self) -> Result<(), String>;
    async fn delete(&self) -> Result<(), String>;
}

fn new_oid() -> ObjectId {
    return ObjectId::new()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LearnrEntry {
    #[serde(default = "new_oid")]
    pub _id: ObjectId,
    pub learned: Vec<String>,
    pub questions: Vec<String>
}

impl LearnrEntry {
    pub fn new(learned: Vec<String>, questions: Vec<String>) -> LearnrEntry {
        return LearnrEntry {
            _id: ObjectId::new(),
            learned, questions
        }
    }
}

#[async_trait]
impl MongoSchema<LearnrEntry> for LearnrEntry {
    async fn insert(&self) -> Result<(), String> {
        let collection = db::connect("entries".to_owned()).await?;
        if let Document(doc) = to_bson(&self).unwrap() {
            let res = collection.insert_one(doc, None).await;
            return match res {
                Ok(_) => Ok(()),
                Err(e) =>  Err(format!("Error inserting entry: {}", e))
            }
        }
        return Err("Could not convert struct to doc".to_owned());
    }
    async fn update(&self) -> Result<(), String> {
        let collection = db::connect("entries".to_owned()).await?;
        if let Document(doc) = to_bson(&self).unwrap() {
            let query = doc!{"_id": self._id.clone()};
            let res = collection.find_one_and_update(query, doc, None).await;
            return match res {
                Ok(_) => Ok(()),
                Err(e) =>  Err(format!("Error updating entry: {}", e))
            }
        }
        return Err("Could not convert struct to doc".to_owned());
    }
    async fn delete(&self) -> Result<(), String> {
        let collection = db::connect("entries".to_owned()).await?;
        let query = doc!{"_id": self._id.clone()};
        let res = collection.delete_one(query, None).await;
        return match res {
            Ok(_) => Ok(()),
            Err(e) =>  Err(format!("Error deleting entry: {}", e))
        }
    }
}

impl RestPath<()> for LearnrEntry {
    fn get_path(_: ()) -> Result<String,restson::Error> { Ok(String::from("entry")) }
}