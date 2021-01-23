
use crate::mongo::user_datasource_mongo::IUserDataSourceMongo;
use async_trait::async_trait;
use domain::repositories::IUserRepository;

pub struct UserRepository<UDSM: IUserDataSourceMongo>{
    user_datasource_mongo: UDSM
} 

impl<UDSM: IUserDataSourceMongo> UserRepository<UDSM> {
    pub fn new(user_datasource_mongo : UDSM) -> Self{
        Self{
            user_datasource_mongo
        }
    }
}

#[async_trait]
impl<UDSM: IUserDataSourceMongo + Sync + Send> IUserRepository  for UserRepository<UDSM> {
    async fn insert(&self, user : &domain::User) -> Result<(), domain::errors::Error> {
       self.user_datasource_mongo.insert(user).await
    }
}