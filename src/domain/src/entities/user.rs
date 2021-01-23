pub struct User{
    pub id: uuid::Uuid,
    pub name: String,
}

impl User {
    pub fn new (id: uuid::Uuid, name: String) -> Self{
        Self{
            id,
            name
        }
    }
}