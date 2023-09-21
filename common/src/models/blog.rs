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
    markdown: String,
    contents: Vec<String>,
}

impl Blog {
    pub fn new(uuid: Uuid,
        title: String,
        person: Uuid,
        project: Option<Uuid>,
        component: Option<Uuid>,
        markdown: String,
        contents: Vec<String>
    ) -> Blog{
        Blog{
            uuid,
            person,
            project,
            component,
            title,
            creation_date: chrono::Utc::now(),
            markdown,
            contents
        }

    }
}
