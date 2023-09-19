use chrono::Utc;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Blog{
    // Links
    uuid: Uuid,
    person: Uuid,
    project: Option<Uuid>,
    component: Option<Uuid>,

    // Blog contents
    title: String,
    creation_date: chrono::DateTime<Utc>, 
    contents: Vec<Uuid>,
}
