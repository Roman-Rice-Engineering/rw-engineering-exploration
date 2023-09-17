use std::ops::Deref;

use rocket::{post, http::CookieJar, State};
use crate::{Session, ManySessions};

#[post("/logout")]
pub async fn auth_logout_post(cookies: &CookieJar<'_>, sessions: &State<ManySessions>, session: Session){
    sessions.deref().delete_session(cookies, &session).await;
}
