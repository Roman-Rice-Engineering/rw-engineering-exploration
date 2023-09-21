use common::models::{base64_files::BlogPost, blog::Blog};
use rocket::{post, State};

use crate::{auth::sessions::Session, env::STORAGE_BUCKET_NAME};


#[post("/create", data = "<data>")]
pub async fn create_blog_post(data: String, session: Session, cloud: &State<cloud_storage::Client>) -> String{
    let failure_message = "unable to create blog post".to_owned();
    println!("{}", &data); 
    let data = match serde_json::from_str::<BlogPost>(&data){
        Ok(c) => c,
        Err(_) => return failure_message
    };
    let files = data.get_files().clone();
    {
    let md_file = data.get_markdown().to_owned().as_bytes().to_vec();
    let md_uuid = uuid::Uuid::new_v4();
    let md_result = match cloud.object().create(STORAGE_BUCKET_NAME, md_file, &md_uuid.to_string(), "text/markdown").await{
            Ok(c) => c,
            Err(e) => {println!("{}", e);return failure_message;}
        };
    }

    

    //let new_db_blog = Blog::new();
    

    failure_message
}
