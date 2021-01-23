use async_trait::async_trait;
use mongodb::bson::doc;
use crate::{connection};

#[async_trait]
pub trait IUserDataSourceMongo {
    async fn insert(&self, user : &domain::entities::User) -> Result<(), domain::errors::Error>;
}

pub struct UserDataSourceMongo;

#[async_trait]
impl IUserDataSourceMongo for UserDataSourceMongo{
    async fn insert(&self, user : &domain::User) -> Result<(), domain::errors::Error> {
        let mongo_collection = connection::MongoClint::get_collection("user").await;

       let doc = doc! {
            "_id": user.id.to_string(),
            "name": user.name.to_string()
       };

      mongo_collection.insert_one(doc, None)
      .await
       .map_err(|e| domain::errors::Error::internal_server_error(e.to_string()))
       .map(|_| ())
      
    }
}
