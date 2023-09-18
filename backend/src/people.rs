use common::auth::Person;
use common::auth::person::PersonBackend;
use mongodb::Collection;
use rocket::futures::TryStreamExt;
use rocket::{post, State};

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
