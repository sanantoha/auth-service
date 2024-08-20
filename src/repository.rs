use mongodb::{Client, bson::doc, Collection};
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
}