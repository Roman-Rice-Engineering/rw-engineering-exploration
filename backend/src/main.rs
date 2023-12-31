mod env;
use auth::login;
use auth::sessions::{ManySessions, Session};
use auth::user_collection::UserCollection;
use cloud_storage::bucket::Cors;
use common::auth::person::PersonBackend;
use common::models::blog::Blog;
use common::models::component;
use common::models::project::Project;
use env::STORAGE_BUCKET_NAME;
use mongodb::bson::Bson;
use mongodb::options::IndexOptions;
use rocket::data::ToByteUnit;
use rocket::{get, routes};
mod auth;
use crate::auth::{signup, logout, profile};
use mongodb::{Client, IndexModel};
mod people;
mod blog;


#[get("/hello")]
fn index(_session: Session) -> String{
    "You are authenticated".to_owned()
}


#[rocket::launch]
async fn rocket() -> _ {

    let cloud_storage = cloud_storage::Client::default();
        
    let db_uri = std::env::var("DB_URI").expect("unable to find 'DB_URI' env variable");
    let db_client = Client::with_uri_str(db_uri).await.expect("unable to connect to database");
    let users = create_users_collection(&db_client).await.expect("unable to create unique index 'username'");
    let sessions = ManySessions::new(db_client.database("auth").collection::<mongodb::bson::Bson>("sessions"));
    let people = create_uuid_indexed_collection::<PersonBackend>(&db_client, "people").await.expect("unable to create people collection");
    let blogs = create_uuid_indexed_collection::<Blog>(&db_client, "blogs").await.expect("unable to create blogs collection");
    let projects = create_uuid_indexed_collection::<Project>(&db_client, "projects").await.expect("unable to create projects collection");
    let components = create_uuid_indexed_collection::<component::Component>(&db_client, "components").await.expect("unable to create components collection");

    let custom_rocket = rocket::data::Limits::default().limit("string", 64.mebibytes());
    let custom_rocket = rocket::Config::figment()
        .merge(("limits", custom_rocket));
    rocket::custom(custom_rocket)
        .manage(users)
        .manage(sessions)
        .manage(people)
        .manage(blogs)
        .manage(projects)
        .manage(components)
        .manage(cloud_storage)
        .mount("/auth", routes![
            index,
            signup::auth_signup_post,
            signup::redirect,
            logout::auth_logout_post,
            profile::auth_profile_post,
            profile::auth_person_post,
            login::auth_login_post,
            login::redirect
        ])
        .mount("/people", routes![
        people::people_index,
        people::people_person
    ]).mount("/blog", routes![crate::blog::create_blog_post, crate::blog::get_blog_post])
}

async fn create_users_collection(db_client: &Client) -> Result<UserCollection, mongodb::error::Error>{
    let options = IndexOptions::builder()
        .unique(true)
        .name("username".to_owned())
        .build();
    let model = IndexModel::builder()
        .keys(mongodb::bson::doc!{"username": "text"})
        .options(options)
        .build();
    match db_client.database("auth").collection::<Bson>("users").create_index(model, None).await{
        Ok(_) => Ok(UserCollection::new(db_client.database("auth").collection::<Bson>("users"))),
        Err(e) => Err(e)
    }
}

async fn create_uuid_indexed_collection<T>(db_client: &Client, name: &str) -> Result<mongodb::Collection<T>, mongodb::error::Error>{
    /*let options = IndexOptions::builder()
        .unique(true)
        .name("uuid".to_owned())
        .build();
    let model = IndexModel::builder()
        .keys(mongodb::bson::doc!{"uuid": "text"})
        .options(options)
        .build();*/
     Ok(db_client.database("semi-public").collection::<T>(&name))/*.create_index(model, None).await{
        Ok(_) => Ok(db_client.database("semi-public").collection::<T>(&name)),
        Err(e) => Err(e)
    }*/
}

