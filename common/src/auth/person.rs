
#[cfg(feature = "database")]
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[cfg(feature = "database")]
use super::user::UserBackend;

#[cfg(feature = "database")]
#[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub struct PersonBackend{
    #[serde(rename = "_id")]
    #[serde(skip_serializing)]
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
            person: Person {first_name, last_name, uuid: uuid::Uuid::new_v4()},
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

    pub fn to_person(self: Self) -> Person{
        Person{
            uuid: self.person.uuid,
            first_name: self.person.get_first_name().to_owned(),
            last_name: self.person.get_last_name().to_owned(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Person{
    uuid: uuid::Uuid,
    first_name: String,
    last_name: String,
}

impl Person{
    pub fn get_uuid(self: &Self) -> &uuid::Uuid{
        &self.uuid
    }
    pub fn get_first_name(self: &Self) -> &str {
        &self.first_name
    }
    pub fn get_last_name(self: &Self) -> &str {
        &self.last_name
    }
    pub fn new(first_name: String, last_name: String) -> Person {
       Person{
            first_name,
            last_name,
            uuid: uuid::Uuid::new_v4(),
        }
    }
}
