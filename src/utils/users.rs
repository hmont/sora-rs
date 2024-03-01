use crate::utils::database;

use mongodb::bson::{doc, Document};

use bcrypt;

pub async fn get_user(api_key: String) -> Option<Document> {
    let collection = database::collection("users").await;
    let cursor = collection.find(doc!{}, None).await.unwrap().deserialize_current();

    while let Ok(ref user) = cursor
    {
        if bcrypt::verify(api_key.clone(), user.get_str("key_hash").unwrap()).unwrap()
        {
            return Some(user.clone());
        }

    }

    None
}