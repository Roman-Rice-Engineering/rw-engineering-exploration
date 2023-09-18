use std::ops::Deref;

use common::auth::Person;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, Html, html, use_effect_with_deps, use_state};
use yew::Properties;

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

    people.deref().iter().map(|person| html!{<OnePerson person={person.clone()} />}).collect()
}

#[derive(Properties, Clone, PartialEq)]
struct OnePersonProps{
    person: Person
}

#[function_component]
fn OnePerson(OnePersonProps { person }: &OnePersonProps) -> Html{
    html!{
        <div>
        <p>{person.get_first_name()}</p>
        <p>{person.get_last_name()}</p>
        </div>
    }
}
