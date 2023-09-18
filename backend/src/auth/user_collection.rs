use common::auth::user::UserBackend;
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

    pub async fn add_user(self: &Self, user: &UserBackend) -> Result<InsertOneResult, &str>{
       match self.users.insert_one(match bson::to_bson(user){
            Ok(c) => c,
            Err(_) => return Err("failed to insert user")
        }, None).await{
            Ok(c) => Ok(c),
            Err(_) => Err("failed to insert user")
        }
    }

    pub async fn get_by_name(self: &Self, name: &str) -> Option<UserBackend>{
        let user: Bson = match self.users.find_one(doc!{"user.username": name}, None).await{
            Ok(c) => match c{
                Some(c) => c,
                None => return None
            },
            Err(_) => return None
        };
        println!("FOUND");
        match bson::from_bson::<UserBackend>(user){
            Err(e) => {println!("{}", e);None},
            Ok(c) => Some(c) 
        }
    }
}
