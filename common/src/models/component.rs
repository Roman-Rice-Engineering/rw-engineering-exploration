use chrono::Utc;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Component{
    // Links
    uuid: Uuid,
    creater: Uuid,
    people: Vec<Uuid>,
    blogs: Vec<Uuid>,
    components: Vec<Uuid>,

    // Blog contents
    state: ComponentState,
    title: String,
    creation_date: chrono::DateTime<Utc>, 
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub enum ComponentState{
    Proposed,
    Started,
    Completed,
}
