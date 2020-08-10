use mongodb::bson::{doc, oid::ObjectId, Bson::Document, to_bson};

mod db;

#[derive(Debug, Serialize)]
pub struct LearnrEntry {
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
    pub async fn insert(&self) -> Result<(), String> {
        let collection = db::connect("entries".to_owned()).await;
        if let Document(doc) = to_bson(&self).unwrap() {
            let res = collection.insert_one(doc, None).await;
            return match res {
                Ok(_) => Ok(()),
                Err(e) =>  Err(format!("Error inserting entry: {}", e))
            }
        }
        return Err("Could not convert struct to doc".to_owned());
    }
    pub async fn update(&self) -> Result<(), String> {
        let collection = db::connect("entries".to_owned()).await;
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
    pub async fn delete(&self) -> Result<(), String> {
        let collection = db::connect("entries".to_owned()).await;
        let query = doc!{"_id": self._id.clone()};
        let res = collection.delete_one(query, None).await;
        return match res {
            Ok(_) => Ok(()),
            Err(e) =>  Err(format!("Error deleting entry: {}", e))
        }
    }
}