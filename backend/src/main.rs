use rocket::{get, routes};
use rocket::http::{CookieJar, Cookie};

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
    rocket::build()
        .mount("/", routes![index])
}
