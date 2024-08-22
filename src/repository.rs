use mongodb::{Client, bson::doc, Collection, bson::Bson};
use mongodb::bson::Document;
use crate::error::Error;
use std::sync::Arc;

#[derive(Debug)]
pub struct UserRepository {
    collection: Collection<Document>
}

impl UserRepository {
    pub fn new(client: Arc<Client>) -> Self {
        let database = client.database("userinfo");
        let collection = database.collection::<Document>("users");

        UserRepository{ collection }
    }

    pub async fn is_valid_user(&self, email: &str, password: &str) -> Result<bool, Error> {
        let filter = doc! { "email": email, "password": password };

        Ok(self.collection.find_one(filter).await?.is_some())
    }

    pub async fn register_user(&self, email: &str, password: &str) -> Result<String, Error> {
        let user = doc! { "email": email, "password": password };
        let insert_res = self.collection.insert_one(user).await?;
        
        match insert_res.inserted_id {
            Bson::ObjectId(oid) => Ok(oid.to_hex()),
            _ => Err(Error::MongoKey(email.to_owned())),
        }        
    }
}