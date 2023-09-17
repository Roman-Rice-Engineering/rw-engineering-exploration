use std::ops::Deref;

use common::{models::DisplayState, auth::{Email, Password, User}};
use web_sys::SubmitEvent;
use yew::{Html, function_component, html, classes, use_state, Callback};
use crate::{auth::auth_form::{FormTextInput, FormSubmitButton, AuthForm}, route::AuthRoute, util::{api_request, auth_reload::auth_reload_login}};
use yew_router::components::Link;

#[function_component]
pub fn Login() -> Html{
    let username_state = use_state(|| String::new());
    let password_state = use_state(|| String::new());

    let username_state_cloned = username_state.clone();
    let username_changed = Callback::from(move |username: String| {
        username_state_cloned.set(username);
    });

    let password_state_cloned = password_state.clone();
    let password_changed = Callback::from(move |email: String| {
        password_state_cloned.set(email);
    });

    // Sign up failure alert.
    let alert_state = use_state(|| DisplayState::Hidden);

    let alert_html = match &*alert_state {
        DisplayState::Hidden => html! {<></>},
        DisplayState::Success { message } => {
            html! {<div class="alert alert-success" role="alert">{"Success: "}{message}</div>}
        }
        DisplayState::Failure { message } => {
            html! {<div class="alert alert-danger" role="alert">{"Failure: "}{message}</div>}
        }
    };

    let alert_state_cloned = alert_state.clone();
    let username_state_cloned = username_state.clone();
    let password_state_cloned = password_state.clone();
    let onsubmit = Callback::from(move |event: SubmitEvent|{
        let alert_state_cloned = alert_state_cloned.clone();
        event.prevent_default();

        password_state_cloned.set(String::new());
        username_state_cloned.set(String::new());
        
        let user_data = User::new(
            username_state_cloned.deref().clone(),
            Email::new(String::new()),
            Password::new_plaintext(password_state_cloned.deref().clone()),
        );

        let user_data = serde_json::to_string(&user_data).unwrap();

        wasm_bindgen_futures::spawn_local(async move {
            let post_submission = match api_request::api_request("/auth/login/", Some(user_data)).await{
                Ok(c) => c,
                Err(_) => String::new()
            };

            alert_state_cloned.set(match serde_json::from_str(&post_submission) {
                Ok(c) => match c{
                    DisplayState::Failure { message } => DisplayState::Failure { message },
                    DisplayState::Hidden => DisplayState::Hidden,
                    DisplayState::Success { .. } => {auth_reload_login(); DisplayState::Hidden}
                },
                Err(e) => DisplayState::Failure {
                    message: e.to_string(),
                },
            });
        });


        
    });



    html!{
        <AuthForm alert={alert_html} title={"Log in"}>
            <form {onsubmit}>
                <FormTextInput name="button1" placeholder="Username" onchange={username_changed} value={username_state.deref().clone()}/>
                <FormTextInput name="button2" placeholder="Password" input_type="password" onchange={password_changed} value={password_state.deref().clone()} />
                <FormSubmitButton />
                <p class="text-center text-muted mt-4">{"Don't have an account? "}<Link<AuthRoute> classes={classes!{"fw-bold"}} to={AuthRoute::Signup}>{"Sign up"}</Link<AuthRoute>></p> 
            </form>
        </AuthForm>
    }
}
