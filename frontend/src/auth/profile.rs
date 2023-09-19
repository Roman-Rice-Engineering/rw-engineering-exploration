use common::auth::{Person, User};
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

    let user = use_state(|| Option::<User>::default());
    {
        let user = user.clone();
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                let response_user = match api_request("/auth/profile/", None).await{
                    Ok(c) => c,
                    Err(_) => return
                };
                let response_user = match serde_json::from_str::<Option<User>>(&response_user){
                    Ok(c) => c,
                    Err(_) => return
                };
                user.set(response_user);
            });
        }, ());
    }

    let user_html = match &*user{
        None => html!{<p>{"Could not fetch user data"}</p>},
        Some(user) => html!{
            <div>
                <p>{"Currently logged in as:\n"}{user.get_username()}</p>
            </div>
        }
    };

    let person_html = match &*person {
        None => html!{<p>{"Account not associated with public profile."}</p>},
        Some(person) => html!{
            <div>
                <p>{"Current public profile: "}</p>
                <p>{"Name: "}{person.get_first_name()}{" "}{person.get_last_name()}</p>
                <p>{"Your current uuid: "}{person.get_uuid().to_string()}</p>
            </div>
            
        }
    }; 

    html!{
        <div>
            {user_html}
            {person_html}
        </div>
    }
}
