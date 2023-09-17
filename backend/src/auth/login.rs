
use common::models::DisplayState;
use rocket::{post, http::CookieJar, State};
use super::{sessions::{Session, ManySessions}, user_collection::UserCollection};

#[post("/login", rank = 1)]
pub fn redirect(_session: Session) -> String{
    match serde_json::to_string(&DisplayState::Failure { message: "you must log out first".to_owned() }){
        Ok(c) => c,
        Err(_) => String::new()
    }
}

// If user is not already logged in go to do login
#[post("/login", data = "<data>", rank = 2)]
pub async fn auth_login_post(data: String, users: &State<UserCollection>, sessions: &State<ManySessions>, cookies: &CookieJar<'_>) -> String{

    let failure_message = serde_json::to_string(&DisplayState::Failure { message: "failed to log in".to_string() }).unwrap().to_string();

    let input_user: common::auth::User = match serde_json::from_str(&data) {
        Ok(value) => value,
        Err(_) => return failure_message
    };
    let user: common::auth::User = match users.get_by_name(&input_user.get_username()).await {
       Some(c) => c,
        None => return failure_message
    };
    let plaintext_password = match input_user.get_plaintext_password() {
        Some(c) => c,
        None => return failure_message
    };

    let verification = match user.verify(&plaintext_password){
        Ok(c) => c,
        Err(_) => return failure_message
    };
    
    if verification == false{
       return failure_message 
    }else if verification == true{
    let session = Session::new(user);
    let _ = session.push_session_to_cookies(cookies);
    let _ = sessions.add_session(session).await;
    }
    // Success response
    serde_json::to_string(&DisplayState::Success { message: "successfully created user".to_owned() }).unwrap()
}

