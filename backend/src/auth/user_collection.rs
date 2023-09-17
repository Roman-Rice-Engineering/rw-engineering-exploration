use crate::User;
use mongodb::{Collection, results::InsertOneResult, bson::{doc, Bson}, bson};

pub struct UserCollection{
    users: Collection<Bson>,
}

impl UserCollection {
    pub fn new(collection: Collection<Bson>) -> UserCollection{
        UserCollection{
            users: collection
        }
    }

    pub async fn add_user(self: &Self, user: &User) -> Result<InsertOneResult, &str>{
       match self.users.insert_one(match bson::to_bson(user){
            Ok(c) => c,
            Err(_) => return Err("failed to insert user")
        }, None).await{
            Ok(c) => Ok(c),
            Err(_) => Err("failed to insert user")
        }
    }

    pub async fn get_by_name(self: &Self, name: &str) -> Option<User>{
        let user: Bson = match self.users.find_one(doc!{"username": name}, None).await{
            Ok(c) => match c{
                Some(c) => c,
                None => return None
            },
            Err(_) => return None
        };
        match bson::from_bson::<User>(user){
            Err(_) => None,
            Ok(c) => Some(c) 
        }
    }
}
