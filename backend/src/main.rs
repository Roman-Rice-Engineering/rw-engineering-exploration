use rocket::{get, routes};
use rocket::http::{CookieJar, Cookie};
mod auth;
use crate::auth::auth_post;
use mongodb::Client;


#[get("/hello")]
fn index(cookies: &CookieJar<'_>) -> String{
    
    let mut read_cookie: String = match cookies.get("helloworld").map(|c| format!("{}", c.value())){
        Some(c) => c,
        None => "Error".to_owned()
    };
    read_cookie.push('a');
    let mut to_ret = "Hello world: ".to_owned();
    let new_cookie = Cookie::new("helloworld", read_cookie.clone());
    cookies.add(new_cookie);
    to_ret.push_str(&read_cookie);
    to_ret
}


#[rocket::launch]
async fn rocket() -> _ {
    
    let db_uri = std::env::var("DB_URI").expect("unable to find 'DB_URI' env variable");
    let db_client = Client::with_uri_str(db_uri).await.expect("unable to connect to database");

    rocket::build()
        .manage(db_client)
        .mount("/", routes![index, auth_post])
}
