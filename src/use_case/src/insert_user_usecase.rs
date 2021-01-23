use async_trait::async_trait;
#[async_trait]
pub trait IInsertUserUseCase {
    async fn execult(&self, name: &str) -> Result<(), domain::errors::Error>;
}

pub struct InsertUserUseCase<'a, UR: domain::repositories::IUserRepository>{
    user_repository: &'a UR
}

impl<'a, UR: domain::repositories::IUserRepository> InsertUserUseCase<'a, UR> {
    pub fn new(user_repository: &'a UR) -> Self{
        Self{
            user_repository
        }
    }
}

#[async_trait]
impl<UR: domain::repositories::IUserRepository + Send + Sync> IInsertUserUseCase for InsertUserUseCase<'_, UR>{
    async fn execult(&self, name: &str) -> Result<(), domain::errors::Error> {

        let user = domain::User{
            id: uuid::Uuid::new_v4(),
            name: name.to_string()
        };

       self.user_repository.insert(&user).await
    }
}