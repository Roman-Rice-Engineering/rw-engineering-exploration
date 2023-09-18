
#[cfg(feature = "database")]
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[cfg(feature = "database")]
use super::user::UserBackend;

#[cfg(feature = "database")]
#[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub struct PersonBackend{
    #[serde(alias = "_id")]
    id: Option<ObjectId>,
    person: Person,
    projects: Vec<ObjectId>,
    blogs: Vec<ObjectId>,
    components: Vec<ObjectId>,
    user: ObjectId,
}

#[cfg(feature = "database")]
impl PersonBackend {
    pub fn new(first_name: String, last_name: String, user: UserBackend) -> Option<PersonBackend>{
        Some(PersonBackend{
            person: Person {first_name, last_name},
            id: None,
            projects: Vec::new(),
            blogs: Vec::new(),
            components: Vec::new(),
            user:  match user.get_id(){
                Some(c) => c,
                None => return None
            }
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Person{
    first_name: String,
    last_name: String,
}

impl Person{
    pub fn new(first_name: String, last_name: String) -> Person {
       Person{
            first_name,
            last_name,
        }
    }
}
