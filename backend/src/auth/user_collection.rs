use crate::User;
use mongodb::{Collection, results::InsertOneResult};

pub struct UserCollection{
    users: Collection<User>,
}

impl UserCollection {
    pub fn new(collection: Collection<User>) -> UserCollection{
        UserCollection{
            users: collection
        }
    }

    pub async fn add_user(self: &Self, user: &User) -> Result<InsertOneResult, &str>{
        match user.is_valid_with_plaintext_password(){
            Ok(()) => (),
            Err(_) => return Err("invalid user")
        };
       match self.users.insert_one(user, None).await{
            Ok(c) => Ok(c),
            Err(_) => Err("failed to insert user")
        }
    }
}
