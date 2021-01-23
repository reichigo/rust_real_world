
use async_trait::async_trait;

use crate::{entities, errors::{Error}};

#[async_trait]
pub trait IUserRepository {
   async fn insert(&self, user : &entities::User) -> Result<(), Error>;
}