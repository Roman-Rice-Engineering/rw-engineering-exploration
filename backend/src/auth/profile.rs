use super::{sessions::Session, user_collection::UserCollection};
use common::auth::person::PersonBackend;
use mongodb::Collection;
use rocket::{post, State};


#[post("/profile")]
pub async fn auth_profile_post(session: Session) -> String{
    match serde_json::to_string(session.get_user()) {
        Ok(c) => c,
        Err(_) => String::new()
    }
}

#[post("/person")]
pub async fn auth_person_post(
    session: Session, 
    users: &State<UserCollection>,
    people: &State<Collection<PersonBackend>>
) -> String{
    let failure_message = "unable to find person".to_owned();
    let person_backend = match session.get_person_backend(users, people).await{
        Some(c) => c,
        None => return failure_message
    };
    let person = person_backend.to_person();
    match serde_json::to_string_pretty(&person){
        Ok(c) => c,
        Err(_) => failure_message
    }
}
