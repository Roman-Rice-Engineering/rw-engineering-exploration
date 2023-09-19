use common::auth::{User, Person, person::PersonBackend};
use rocket::http::{CookieJar, Cookie};
use uuid::Uuid;
use crate::env::IS_PRODUCTION;
use serde::{Serialize, Deserialize};
use mongodb::{bson, Collection};

use super::user_collection::UserCollection;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session{
    user: User,
    session_id: Uuid,
    csrf_token: Uuid,
    expiration: chrono::DateTime<chrono::Utc>
}

impl Session{
    pub fn new(user: User) -> Session{
        let user = user.washout_password();
        Session{
            user,
            session_id: Uuid::new_v4(),
            csrf_token: Uuid::new_v4(),
            expiration: chrono::Utc::now() + chrono::Duration::days(1),
        }
    }

    pub fn get_user(self: &Self) -> &User {
        &self.user
    }

    pub async fn get_person_backend(self: &Self, users: &UserCollection, people: &Collection<PersonBackend>) -> Option<PersonBackend>{
        let username = self.get_user().get_username();
        let user = users.get_by_name(username).await?;
        match people.find_one(bson::doc!{"_id": user.get_person_id()?}, None).await{
            Ok(c) => c,
            Err(_) => None
        }
    }
/*
    pub fn get_session_id(self: &Self) -> &Uuid{
        &self.session_id
    }
*/
    pub fn push_session_to_cookies(self: &Self, cookies: &CookieJar<'_>) -> Result<(), serde_json::Error>{
        let cookie_sid = Cookie::build("SID", match serde_json::to_string(&self.session_id){
            Ok(c) => c,
            Err(e) => return Err(e)
        })
            .http_only(true)
            .same_site(rocket::http::SameSite::Strict)
            .secure(IS_PRODUCTION)
            .finish();
        cookies.add(cookie_sid);

        let cookie_csrf = Cookie::build("CSRF_TOKEN", match serde_json::to_string(&self.csrf_token){
            Ok(c) => c,
            Err(e) => return Err(e)
        })
            .http_only(false)
            .same_site(rocket::http::SameSite::Strict)
            .secure(IS_PRODUCTION)
            .finish();
        cookies.add(cookie_csrf);
        Ok(())
    }

    pub fn verify_by_csfr_token(self: &Self, token: Uuid) -> bool{
       token ==  self.csrf_token
    }
}

#[derive(Debug)]
pub struct ManySessions{
    sessions: Collection<bson::Bson>
}

impl ManySessions{
    pub async fn get_session_from_cookies_and_csrf_token(self: &Self, cookies: &CookieJar<'_>, csrf_token: Uuid) -> Result<Option<Session>, serde_json::Error>{
        let cookie_sid = match cookies.get("SID"){
            Some(c) => c,
            None => return Ok(None)
        };
        let cookie_csrf = match cookies.get("CSRF_TOKEN"){
            Some(c) => c,
            None => return Ok(None)
        };

        let cookie_csrf_as_uuid = serde_json::from_str::<Uuid>(&cookie_csrf.value())?; 
        let cookie_sid_as_uuid = serde_json::from_str::<Uuid>(&cookie_sid.value())?;

        if cookie_csrf_as_uuid != csrf_token{
            return Ok(None);
        }

        let session = match self.get_session_by_session_id(cookie_sid_as_uuid).await{
            Some(s) => s,
            None => return Ok(None)
        };

        if session.verify_by_csfr_token(cookie_csrf_as_uuid) == false{
            return Ok(None);
        }

        Ok(Some(session))
    }

    pub async fn add_session(self: &Self, session: Session) -> Result<mongodb::results::InsertOneResult, mongodb::error::Error>{
        self.sessions.insert_one(mongodb::bson::to_bson(&session).unwrap(), None).await
    }

    pub fn new(col: Collection<bson::Bson>) -> ManySessions{
        ManySessions{
            sessions: col
        }
    }

    /*pub async fn delete_outdated_sessions(self: &mut Self){
        let now = chrono::Utc::now();
        self.sessions.retain(|e| now < e.expiration);
    }

    pub async fn delete_sessions_by_user(self: &mut Self, user: User){
        self.sessions.retain(|e| user != e.user);
    }

    pub async fn delete_sessions_by_session_id(self: &mut Self, session_id: Uuid){
        self.sessions.retain(|e| session_id != e.session_id);
    }*/

    pub async fn delete_session(self: &Self, cookies: &CookieJar<'_>, session: &Session){
        let _ = self.sessions.delete_one(bson::doc!{
            "session_id": bson::to_bson(&session.session_id).unwrap(),
            "csrf_token": bson::to_bson(&session.csrf_token).unwrap()
        }, None).await;
        cookies.remove(Cookie::new("SID", ""));
        cookies.remove(Cookie::new("CSRF_TOKEN", ""));
    }

    pub async fn get_session_by_session_id(self: &Self, session_id: Uuid) -> Option<Session>{
        let session_id_serialized = match bson::to_bson(&session_id){
            Ok(c) => c,
            Err(_) => return None
        };
        let bson = match self.sessions.find_one(mongodb::bson::doc!{"session_id": session_id_serialized}, None).await{
            Err(_) => return None,
            Ok(c) => match c{None => return None, Some(c) => c}
        };
        Some(match bson::from_bson::<Session>(bson){Err(_) => return None, Ok(c) => c})
    }
}
