use super::sessions::Session;
use rocket::post;


#[post("/profile")]
pub async fn auth_profile_post(session: Session) -> String{
    match serde_json::to_string(session.get_user()) {
        Ok(c) => c,
        Err(_) => String::new()
    }
}
