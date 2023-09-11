use rocket::{get, routes};

#[get("/")]
fn index() -> String{
    "Hello world!".to_owned()
}

#[rocket::launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}
