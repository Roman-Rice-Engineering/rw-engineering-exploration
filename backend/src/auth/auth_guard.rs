use crate::Session;
use mongodb::Client;
use rocket::http::Status;
use rocket::request::{Request, FromRequest};
use rocket::request;
use uuid::Uuid;

use super::sessions::ManySessions;

#[derive(Debug)]
pub enum SessionAuthError{
    NotAuthenticated,
    DatabaseAccessError,
    NoSessionsFound
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session{
    type Error = SessionAuthError;
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error>{
        let cookies = request.cookies();
        let sessions = match request.rocket().state::<ManySessions>(){
            Some(c) => c,
            None => {println!("FAIL AT SESSIONS");return request::Outcome::Failure((Status::Forbidden, SessionAuthError::NoSessionsFound))}
        };
        println!("{}", cookies.get("SID").unwrap().value());
        let session_id = serde_json::from_str::<Uuid>(cookies.get("SID").unwrap().value()).unwrap();

        let session = sessions.get_session_by_session_id(session_id).await;
        match session {
            Some(c) => request::Outcome::Success(c),
            None => request::Outcome::Failure((Status::Forbidden, SessionAuthError::NotAuthenticated))
        }

    }

}
