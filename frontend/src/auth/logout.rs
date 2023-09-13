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
                let fetched_data: String = Request::get(&(API_URL.to_owned() + "hello/"))
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                data.set(fetched_data);
            });
        }, ());

    }


    html!{
        <p>
            {data.to_string()}
        </p>
        
    }
}
