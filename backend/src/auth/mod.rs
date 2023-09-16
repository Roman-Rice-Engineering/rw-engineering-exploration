use rocket::State;

use rocket::post;
use common::{auth::user::User, models::DisplayState};

#[post("/auth/signup", data = "<data>")]
pub async fn auth_post(data: String, client: &State<mongodb::Client>) -> String{

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
    let db = client.database("auth");
    let col: mongodb::Collection<User> = db.collection("users");
    let _ = col.insert_one(user, None).await;

    // Success response
    serde_json::to_string(&DisplayState::Success { message: "successfully created user".to_owned() }).unwrap()
}
