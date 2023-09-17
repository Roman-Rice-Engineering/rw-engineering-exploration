use yew::{function_component, Html, html, use_state, use_effect_with_deps};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use crate::env::API_URL;

#[function_component]
pub fn Logout() -> Html{
    
    let data = use_state(|| String::new());
    {
        let data = data.clone();
        use_effect_with_deps(move |_| {
            let data = data.clone();
            spawn_local(async move {
                let fetched_data: String = Request::post(&(API_URL.to_owned() + "auth/logout/"))
                    .header("X-CSRF-Token", &wasm_cookies::get("CSRF_TOKEN").unwrap().unwrap())
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
            });
        }, ());

    }


    html!{
        <p>
            {"You are now logged out!"}
        </p>
        
    }
}
