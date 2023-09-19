use common::auth::Person;
use yew::{function_component, html, Html, use_state, use_effect_with_deps, platform::spawn_local};

use crate::util::api_request::api_request;



#[function_component]
pub fn Profile() -> Html{

   let person = use_state(|| Option::<Person>::default());
    {
        let person = person.clone();
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                let response_person = match api_request("/auth/person/", None).await{
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

    html!{
        <div>
            <p>{"Currently logged in as: "}</p>
            {match serde_json::to_string(&*person){
                Ok(c) => c,
                Err(_) => String::new()
            }}
        </div>
    }
}
