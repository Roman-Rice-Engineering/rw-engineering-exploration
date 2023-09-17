mod env;
use auth::login;
use auth::sessions::{ManySessions, Session};
use auth::user_collection::UserCollection;
use mongodb::bson::Bson;
use mongodb::options::IndexOptions;
use rocket::{get, routes};
mod auth;
use crate::auth::{signup, logout, profile};
use mongodb::{Client, IndexModel};
use common::auth::User;


#[get("/hello")]
fn index(_session: Session) -> String{
    "You are authenticated".to_owned()
}


#[rocket::launch]
async fn rocket() -> _ {
    
    let db_uri = std::env::var("DB_URI").expect("unable to find 'DB_URI' env variable");
    let db_client = Client::with_uri_str(db_uri).await.expect("unable to connect to database");
    let users = create_users_collection(&db_client).await.expect("unable to create unihelloque index 'username'");
    let sessions = ManySessions::new(db_client.database("auth").collection::<mongodb::bson::Bson>("sessions"));

    rocket::build()
        .manage(users)
        .manage(sessions)
        .mount("/auth", routes![
            index,
            signup::auth_signup_post,
            signup::redirect,
            logout::auth_logout_post,
            profile::auth_profile_post,
            login::auth_login_post,
            login::redirect
        ])
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

