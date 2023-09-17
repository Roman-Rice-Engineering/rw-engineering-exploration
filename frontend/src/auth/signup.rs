use std::ops::Deref;

use crate::auth::auth_form::{AuthForm, FormTextInput, FormSubmitButton};
use crate::env::API_URL;
use crate::route::AuthRoute;
use common::auth::{User, Password, Email};
use common::models::DisplayState;
use gloo_net::http::Request;
use web_sys::SubmitEvent;
use yew::{classes, function_component, html, use_state, Callback, Html};
use yew_router::prelude::*;

#[function_component]
pub fn Signup() -> Html {
    let username_state = use_state(|| String::new());
    let email_state = use_state(|| String::new());
    let password1_state = use_state(|| String::new());
    let password2_state = use_state(|| String::new());

    let username_state_cloned = username_state.clone();
    let username_changed = Callback::from(move |username: String| {
        username_state_cloned.set(username);
    });

    let email_state_cloned = email_state.clone();
    let email_changed = Callback::from(move |email: String| {
        email_state_cloned.set(email);
    });

    let password1_state_cloned = password1_state.clone();
    let password1_changed = Callback::from(move |password: String| {
        password1_state_cloned.set(password);
    });

    let password2_state_cloned = password2_state.clone();
    let password2_changed = Callback::from(move |password: String| {
        password2_state_cloned.set(password);
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
    let email_state_cloned = email_state.clone();
    let password1_state_cloned = password1_state.clone();
    let password2_state_cloned = password2_state.clone();
    let onsubmit = Callback::from(move |event: SubmitEvent| {
        let alert_state_cloned = alert_state_cloned.clone();
        event.prevent_default();

        let mut form_good: DisplayState = DisplayState::Hidden;

        let password = password1_state_cloned.deref().clone();

        if username_state_cloned.deref() == "" {
            form_good = DisplayState::Failure {
                message: "username cannot be empty".to_owned(),
            }
        }
        if email_state_cloned.deref() == "" {
            form_good = DisplayState::Failure {
                message: "email cannot be empty".to_owned(),
            }
        }
        if password1_state_cloned.deref() == "" {
            form_good = DisplayState::Failure {
                message: "password cannot be empty".to_owned(),
            }
        }
        if password2_state_cloned.deref() == "" {
            form_good = DisplayState::Failure {
                message: "confirm password cannot be empty".to_owned(),
            }
        }
        if password2_state_cloned.deref() != password1_state_cloned.deref() {
            form_good = DisplayState::Failure {
                message: "passwords do not match".to_owned(),
            }
        }

        password1_state_cloned.set(String::new());
        password2_state_cloned.set(String::new());
        email_state_cloned.set(String::new());
        username_state_cloned.set(String::new());

        match form_good {
            DisplayState::Hidden => (),

            // At this point 'Success' should not be a possible state
            // because we did not yet send data to the backend
            DisplayState::Success { message: _ } => (),

            DisplayState::Failure { message } => {
                alert_state_cloned.set(DisplayState::Failure { message });
                return;
            }
        }

        let user_data = User::new(
            username_state_cloned.deref().clone(),
            Email::new(email_state_cloned.deref().clone()),
            Password::new_plaintext(password),
        );
        let user_data = serde_json::to_string_pretty(&user_data).unwrap();
        wasm_bindgen_futures::spawn_local(async move {
            let post_submission: String = Request::post(&(API_URL.to_owned() + "auth/signup/"))
                .header("X-CSRF-Token", match wasm_cookies::get("CSRF_TOKEN"){
                    Some(c) => match c {
                        Ok(s) => s,
                        Err(_) => "".to_owned()
                    },
                    None => "".to_owned()
                }.deref())
                .body(user_data)
                .unwrap()
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            alert_state_cloned.set(match serde_json::from_str(&post_submission) {
                Ok(c) => c,
                Err(e) => DisplayState::Failure {
                    message: e.to_string(),
                },
            });
        });
    });

    html! {
        <AuthForm alert={alert_html}>
            <form {onsubmit}>
                <FormTextInput name="button1" placeholder="Username" onchange={username_changed} value={username_state.deref().clone()}/>
                <FormTextInput name="button2" placeholder="email@example.com" input_type="email" onchange={email_changed} value={email_state.deref().clone()} />
                <FormTextInput name="password1" placeholder="Password" input_type="password" onchange={password1_changed} value={password1_state.deref().clone()} />
                <FormTextInput name="password2" placeholder="Confirm Password" input_type="password" onchange={password2_changed} value={password2_state.deref().clone()} />
                <FormSubmitButton />
                <p class="text-center text-muted mt-4">{"Already have an account? "}<Link<AuthRoute> classes={classes!{"fw-bold"}} to={AuthRoute::Login}>{"Log in"}</Link<AuthRoute>></p>
            </form>
        </AuthForm>
    }
}
