use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub struct User{
    id: Option<ObjectId>,
    username: String,
    first_name: String,
    last_name: String,
    email: String,
    password: Option<String>,
}


