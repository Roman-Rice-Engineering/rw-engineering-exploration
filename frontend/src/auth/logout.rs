use yew::{function_component, Html, html, use_state, use_effect_with_deps};
use wasm_bindgen_futures::spawn_local;
use crate::lib::api_request::api_request;

#[function_component]
pub fn Logout() -> Html{
    
    let data = use_state(|| String::new());
    {
        let data = data.clone();
        use_effect_with_deps(move |_| {
            let data = data.clone();
            spawn_local(async move {
                let _ = api_request("/auth/logout/", None).await;
            });
        }, ());

    }


    html!{
        <p>
            {"You are now logged out!"}
        </p>
        
    }
}
