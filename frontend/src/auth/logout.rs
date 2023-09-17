use yew::{function_component, Html, html, use_effect_with_deps};
use wasm_bindgen_futures::spawn_local;
use crate::util::{api_request::api_request, auth_reload::auth_reload_logout};

#[function_component]
pub fn Logout() -> Html{
    
    {
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                let _ = api_request("/auth/logout/", None).await;
            });
            auth_reload_logout();
        }, ());

    }


    html!{
        <p>
            {"You are now logged out!"}
        </p>
        
    }
}
