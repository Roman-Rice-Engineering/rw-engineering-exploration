use std::ops::Deref;

use common::models::DisplayState;
use rocket::{http::CookieJar, State, post};
use common::auth::user::User;
use crate::auth::sessions::Session;

use super::{user_collection::UserCollection, sessions::ManySessions};


// If user is signed in go to a failure message telling them that they need to log out before
// creating a new user
#[post("/signup", rank = 1)]
pub fn redirect(_session: Session) -> String{
    match serde_json::to_string(&DisplayState::Failure { message: "you must log out first".to_owned() }){
        Ok(c) => c,
        Err(_) => String::new()
    }
}

// If user is not already logged in go to do signup
#[post("/signup", data = "<data>", rank = 2)]
pub async fn auth_signup_post(data: String, users: &State<UserCollection>, sessions: &State<ManySessions>, cookies: &CookieJar<'_>) -> String{

    let failure_message = serde_json::to_string(&DisplayState::Failure { message: "failed to create user".to_string() }).unwrap().to_string();

    let mut user: User = match serde_json::from_str(&data) {
        Ok(value) => value,
        Err(_) => return failure_message
    };
    match user.is_valid_with_plaintext_password() {
        Ok(()) => (),
        Err(_) => return failure_message
    }
    match user.hash() {
        Ok(()) => (),
        Err(_) => return failure_message
    }
    let _user_oid = match users.deref().add_user(&user).await{
        Ok(c) => c.inserted_id.as_object_id().unwrap(),
        Err(_) => return serde_json::to_string(&DisplayState::Failure { message: "user with that name already exists".to_string() }).unwrap().to_string()

    };
    let session = Session::new(user);
    let _ = session.push_session_to_cookies(cookies);
    let _ = sessions.add_session(session).await;

    // Success response
    serde_json::to_string(&DisplayState::Success { message: "successfully created user".to_owned() }).unwrap()
}

