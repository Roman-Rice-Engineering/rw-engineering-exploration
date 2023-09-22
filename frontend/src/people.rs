use std::ops::Deref;
use yew_router::prelude::Link;

use common::auth::Person;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, Html, html, use_effect_with_deps, use_state};
use yew::Properties;

use crate::route::PeopleRoute;
use crate::util::api_request::api_request;


#[function_component]
pub fn People() -> Html{

    let people = use_state(|| Vec::<Person>::new());
    
    {
        let people = people.clone();
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                let response_people = match api_request("/people/", None).await{
                    Ok(c) => c,
                    Err(_) => return
                };
                let response_people = match serde_json::from_str::<Vec<Person>>(&response_people){
                    Ok(c) => c,
                    Err(_) => return
                };
                people.set(response_people);
            });
        }, ());

    }

    let people_html: Html = people.deref().iter().map(|person| html!{
        <OnePerson person={person.clone()} />
        
    }).collect();

    html!{
        <div class="container">
            <div class="row row-cols-1 row-cols-sm-2 row-cols-md-3 g-3 p-0 m-0">
                {people_html} 
            </div>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct OnePersonProps{
    pub person: Person
}

#[function_component]
fn OnePerson(OnePersonProps { person }: &OnePersonProps) -> Html{
    html!{
        <div class="col">
            <div class="card shadow-sm">
                <Link<PeopleRoute> to={PeopleRoute::Person { uuid: person.get_uuid().to_string() }} classes="image-zoom-hover-wrapper">
                    <img class="card-img-top image-zoom-hover" src="https://loremflickr.com/640/640" focusable="false" />
                </Link<PeopleRoute>>
                <div class="card-body">
                    <p class="card-text">{person.get_first_name()}{" "}{person.get_last_name()}</p>
                    <div class="d-flex justify-content-between align-items-center">
                        <form method="post" action="#">
                            <div class="btn-group">
                                <Link<PeopleRoute> to={PeopleRoute::Person {uuid: person.get_uuid().to_string()}} classes="btn btn-outline-primary">{"More Information"}</Link<PeopleRoute>>
                            </div>
                        </form>
                        <small class="text-muted"></small>
                    </div>
                    </div>
                </div>
            </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct PeoplePersonProps{
    pub uuid: String
}

#[function_component]
pub fn PeoplePerson(PeoplePersonProps { uuid }: &PeoplePersonProps) -> Html{
    let person = use_state(|| Option::<Person>::default());
    {
        let person = person.clone();
        let path = "/people/".to_owned() + uuid;
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                let response_person = match api_request(&path, None).await{
                    Ok(c) => c,
                    Err(_) => return
                };
                let response_person = match serde_json::from_str::<Option<Person>>(&response_person){
                    Ok(c) => c,
                    Err(_) => return
                };
                person.set(response_person);
            });
        }, ());
    }
    match &*person {
        None => html!{},
        Some(person) => {
        let blog_post_html: Html = person.get_blogs().iter().map(|blog| html!{<p>{"Blog post:  "}{blog.to_string()}</p>}).collect();
        html!{
            <div>
                <p>{"This is name of person: "}{person.get_first_name()}</p>
                <p>{"This is his uuid: "}{person.get_uuid().to_string()}</p>
                {blog_post_html}
            </div>
        }
        }
    }

}
