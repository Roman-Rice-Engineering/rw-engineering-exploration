use std::ops::Deref;

use crate::env::API_URL;
use crate::route::AuthRoute;
use common::auth::{User, Password, Email};
use common::models::DisplayState;
use gloo_net::http::Request;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, SubmitEvent};
use yew::{classes, function_component, html, use_state, Callback, Event, Html, Properties};
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
            let post_submission: String = Request::post(&(API_URL.to_owned() + "auth/"))
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
        <section style="height: 90vh;" class="bg-caution">
            <div class="container h-100">
              <div class="row d-flex justify-content-center align-items-center h-100">
                <div class="col-12 col-md-8 col-lg-6 col-xl-5">
                  <div class="card" style="border-radius: 25px;">
                    <div class="card-body p-5">
                      {alert_html}
                      <h2 class="text-center mb-5">{"Create an account"}</h2>
                      <form {onsubmit}>
                        {&*username_state}
                        <FormTextInput name="button1" placeholder="Username" onchange={username_changed} value={username_state.deref().clone()}/>
                        {&*email_state}
                        <FormTextInput name="button2" placeholder="email@example.com" input_type="email" onchange={email_changed} value={email_state.deref().clone()} />
                        {&*password1_state}
                        <FormTextInput name="password1" placeholder="Password" input_type="password" onchange={password1_changed} value={password1_state.deref().clone()} />
                        {&*password2_state}
                        <FormTextInput name="password2" placeholder="Confirm Password" input_type="password" onchange={password2_changed} value={password2_state.deref().clone()} />
                        <FormSubmitButton />
                        <p class="text-center text-muted mt-4">{"Already have an account? "}<Link<AuthRoute> classes={classes!{"fw-bold"}} to={AuthRoute::Login}>{"Log in"}</Link<AuthRoute>></p>
                      </form>
                    </div>
                  </div>
                </div>
              </div>
            </div>
        </section>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct FormTextInputProps {
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or("text".to_owned())]
    pub input_type: String,
    pub name: String,
    pub onchange: Callback<String>,
    pub value: String,
}

#[function_component]
fn FormTextInput(
    FormTextInputProps {
        value,
        placeholder,
        name,
        input_type,
        onchange,
    }: &FormTextInputProps,
) -> Html {
    let onchange = onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        onchange.emit(value);
    });
    html! {
        <div class="form-outline mb-4">
            <input value={value.clone()} {onchange} type={input_type.clone()} placeholder={placeholder.clone()} name={name.clone()} class="form-control form-control-lg" />
        </div>

    }
}

#[function_component]
fn FormSubmitButton() -> Html {
    html! {
            <div class="d-flex justify-content-center">
                <button type="submit" class="text-body btn btn-success btn-lg">{"Submit form"}</button>
            </div>

    }
}
