pub mod user_collection;
pub mod sessions;
pub mod auth_guard;
use std::ops::Deref;

use rocket::State;

use rocket::http::CookieJar;
use rocket::post;
use common::{auth::user::User, models::DisplayState};

use self::sessions::{ManySessions, Session};
use self::user_collection::UserCollection;

#[post("/signup", rank = 1)]
pub fn redirect(session: Session) -> String{
    serde_json::to_string(&DisplayState::Failure { message: "you must log out first".to_owned() }).unwrap()
}

#[post("/signup", data = "<data>", rank = 2)]
pub async fn auth_signup_post(data: String, users: &State<UserCollection>, sessions: &State<ManySessions>, cookies: &CookieJar<'_>) -> String{

    let failure_message = serde_json::to_string(&DisplayState::Failure { message: "failed to create user".to_string() }).unwrap().to_string();

    let mut user: User = match serde_json::from_str(&data) {
        Ok(value) => value,
        Err(_) => return failure_message
    };
    match user.is_valid_with_plaintext_password() {
        Ok(()) => (),
        Err(e) => {println!("Error: {}", e); return failure_message;}
    }
    match user.hash() {
        Ok(()) => (),
        Err(e) => {println!("Error: {}", e); return failure_message;}
    }
    let _user_oid = match users.deref().add_user(&user).await{
        Ok(c) => {println!("{}", c.inserted_id.to_string()); c.inserted_id.as_object_id().unwrap()},
        Err(e) => {println!("Error: {}", e); return failure_message;}
    };
    let session = Session::new(user);
    let _ = session.push_session_to_cookies(cookies);
    let result = sessions.add_session(session).await;
    println!("{}", match result{
        Ok(c) => c.inserted_id.to_string(),
        Err(e) => e.to_string()
    });

    // Success response
    serde_json::to_string(&DisplayState::Success { message: "successfully created user".to_owned() }).unwrap()
}

/*
#[post("/logout")]
pub async fn auth_logout_post(sessions: &State<ManySessions>, session: Session){
    sessions.deref().
    
}*/
