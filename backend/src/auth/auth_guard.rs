use crate::Session;
use rocket::http::Status;
use rocket::request::{Request, FromRequest};
use rocket::request;
use uuid::Uuid;

use super::sessions::ManySessions;

#[derive(Debug)]
pub enum SessionAuthError{
    //NotAuthenticated,
    //DatabaseAccessError,
    NoSessionsFound
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session{
    type Error = SessionAuthError;
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error>{
        let cookies = request.cookies();
        let sessions = match request.rocket().state::<ManySessions>(){
            Some(c) => c,
            None => return request::Outcome::Failure((Status::Forbidden, SessionAuthError::NoSessionsFound))
        };
        let csrf_token = request.headers().get("X-CSRF-Token").next().unwrap();
        let csrf_token = serde_json::from_str::<Uuid>(csrf_token).unwrap();
        let session = sessions.get_session_from_cookies_and_csrf_token(cookies, csrf_token).await;
        match session {
            Ok(c) => request::Outcome::Success(match c {
                Some(c) => c,
                None => return request::Outcome::Forward(())
            }),
            Err(_) => request::Outcome::Forward(())
        }

    }

}
