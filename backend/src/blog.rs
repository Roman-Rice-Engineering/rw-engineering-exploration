use rocket::post;

use crate::auth::sessions::Session;


#[post("/create", data = "<data>")]
pub fn create_blog_post(data: String, session: Session) -> String{
    let failure_message = "unable to create blog post".to_owned();
    println!("{}", data); 
    

    failure_message
}
