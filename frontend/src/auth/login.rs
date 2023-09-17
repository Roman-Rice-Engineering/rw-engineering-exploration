use std::ops::Deref;

use yew::{Html, function_component, html, classes, use_state, Callback};
use crate::{auth::auth_form::{FormTextInput, FormSubmitButton, AuthForm}, route::AuthRoute};
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


    html!{
        <AuthForm alert={html!{}} title={"Log in"}>
            <form>
                <FormTextInput name="button1" placeholder="Username" onchange={username_changed} value={username_state.deref().clone()}/>
                <FormTextInput name="button2" placeholder="Password" input_type="password" onchange={password_changed} value={password_state.deref().clone()} />
                <FormSubmitButton />
                <p class="text-center text-muted mt-4">{"Don't have an account? "}<Link<AuthRoute> classes={classes!{"fw-bold"}} to={AuthRoute::Signup}>{"Sign up"}</Link<AuthRoute>></p> 
            </form>
        </AuthForm>
    }
}
