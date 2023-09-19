use chrono::Utc;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Project{
    // Links
    uuid: Uuid,
    leader: Uuid,
    people: Vec<Uuid>,
    blogs: Vec<Uuid>,
    components: Vec<Uuid>,

    // Blog contents
    title: String,
    creation_date: chrono::DateTime<Utc>, 
}
