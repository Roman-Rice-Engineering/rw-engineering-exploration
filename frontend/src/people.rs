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
    person: Person
}

#[function_component]
fn OnePerson(OnePersonProps { person }: &OnePersonProps) -> Html{
    html!{
        <div class="col">
            <div class="card shadow-sm">
                <a href="/blog/{{blog.id}}" class="image-zoom-hover-wrapper">
                    <img class="card-img-top image-zoom-hover" src="https://loremflickr.com/640/640" focusable="false" />
                </a>
                <div class="card-body">
                    <p class="card-text">{person.get_first_name()}{" "}{person.get_last_name()}</p>
                    <div class="d-flex justify-content-between align-items-center">
                        <form method="post" action="#">
                            <div class="btn-group">
                                <button type="button" class="btn btn-outline-primary">{"More Information"}</button>
                            </div>
                        </form>
                        <small class="text-muted"></small>
                    </div>
                    </div>
                </div>
            </div>
    }
}
