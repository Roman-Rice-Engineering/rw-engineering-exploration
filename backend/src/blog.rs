use base64::Engine;
use common::{models::{base64_files::BlogPost, blog::Blog}, auth::person::PersonBackend};
use mongodb::{Collection, bson::{doc, Bson, Document, Binary, SerializerOptions}, bson};
use rocket::{post, State};
use uuid::Uuid;

use crate::{auth::{sessions::Session, user_collection::UserCollection}, env::STORAGE_BUCKET_NAME};


#[post("/get/<uuid>")]
pub async fn get_blog_post(uuid: String, blogs: &State<Collection<Blog>> ) -> Option<String>{
    let uuid = match Uuid::parse_str(&uuid) {
        Ok(c) => c,
        Err(_) => return None
    };
    let uuid = match mongodb::bson::ser::to_bson_with_options(&uuid, SerializerOptions::builder().human_readable(false).build()){
        Ok(c) => c,
        Err(_) => return None
    };
    let blog_post = match blogs.find_one(doc!{"uuid": &uuid}, None).await{
        Ok(c) => c,
        Err(e) => {
            println!("{}{e}", doc!{"uuid": &uuid});
            return None;
        }
    }; 
    match serde_json::to_string(&blog_post){Ok(c) => Some(c), Err(_) => None}
}

#[post("/create", data = "<data>")]
pub async fn create_blog_post(
    data: String,
    session: Session,
    cloud: &State<cloud_storage::Client>,
    users: &State<UserCollection>,
    people: &State<Collection<PersonBackend>>,
    blogs: &State<Collection<Blog>>
) -> String{
    let failure_message = "unable to create blog post".to_owned();
    println!("{}", &data); 
    let data = match serde_json::from_str::<BlogPost>(&data){
        Ok(c) => c,
        Err(_) => return failure_message
    };
    let files = data.get_files().clone();
    
    let md_file = data.get_markdown().to_owned().as_bytes().to_vec();
    let md_uuid = uuid::Uuid::new_v4();
    let md_result = match cloud.object().create(STORAGE_BUCKET_NAME, md_file, &md_uuid.to_string(), "text/markdown").await{
            Ok(c) => c,
            Err(e) => {println!("{}", e);return failure_message;}
        };

    // Handle link to person

    let new_blog_uuid = uuid::Uuid::new_v4();

    let authenticated_person = match session.get_person_backend(users, people).await{
        Some(c) => c,
        None => return failure_message
    };

    {
    let query = doc!{"_id": authenticated_person.get_id().unwrap()};
    let update = doc!{"$push": {"person.blogs":  Binary::from_base64(base64::engine::general_purpose::STANDARD.encode(new_blog_uuid), None).unwrap()}};
    people.update_one(query, update, None).await;
    }

    
    // Handle database representation of blog
    let new_db_blog = Blog::new(
        new_blog_uuid,
        "Placeholder".to_owned(),
        authenticated_person.to_person().get_uuid().clone(), 
        None,
        None,
        md_uuid,
        Vec::new()
    );

    let new_blog_result = match blogs.insert_one(new_db_blog, None).await{
        Ok(c) => c,
        Err(_) => return failure_message
    };
        
    failure_message
}
