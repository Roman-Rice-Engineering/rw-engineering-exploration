
use common::auth::Person;
use common::auth::person::PersonBackend;
use mongodb::Collection;
use rocket::futures::TryStreamExt;
use rocket::{post, State};
use mongodb::bson::{doc, SerializerOptions};
use uuid::Uuid;

#[post("/")]
pub async fn people_index(people: &State<Collection<PersonBackend>>) -> String{
    let failure_message = "failed to retrieve people data".to_owned();

    let people: Vec<PersonBackend> = match people.find(None, None).await {
        Err(_) => return failure_message,
        Ok(c) => match c.try_collect::<Vec<PersonBackend>>().await{
            Ok(c) => c,
            Err(_) => return failure_message
        }
    };

    let people: Vec<Person> = people.iter().map(|personbackend| personbackend.clone().to_person()).collect();

    match serde_json::to_string(&people){
        Err(_) => failure_message,
        Ok(c) => c
    }
}

#[post("/<uuid>")]
pub async fn people_person(
    uuid: String,
    people: &State<Collection<PersonBackend>>
) -> String{
    let failure_message = "error getting person data".to_owned();
    let uuid = match Uuid::parse_str(&uuid){
        Ok(c) => c,
        Err(_) => return failure_message
    };
    let uuid = match mongodb::bson::ser::to_bson_with_options(&uuid, SerializerOptions::builder().human_readable(false).build()){
        Ok(c) => c,
        Err(_) => return failure_message
    };
    let person = match people.find_one(doc!{"person.uuid": uuid}, None).await{
        Ok(c) => c,
        Err(_) => return failure_message
    };
    let person = match person{
        Some(c) => c,
        None => return failure_message
    };
    let person = person.to_person();
    match serde_json::to_string_pretty(&person){
        Ok(c) => c,
        Err(_) => failure_message
    }
}
