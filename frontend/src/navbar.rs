use std::ops::Deref;

use common::auth::User;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::{Route, route::AuthRoute, util::api_request::api_request};

#[function_component]
pub fn MainNav() -> Html{
    let username = use_state(|| Option::<String>::default());
    {
        let username = username.clone();
        use_effect_with_deps(move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                let response = match api_request("/auth/profile/", None)
                .await{
                    Ok(c) => c,
                    Err(_) => return
                };
                let user = match serde_json::from_str::<User>(&response){
                    Ok(c) => c,
                    Err(_) => return
                };
                username.set(Some(user.get_username().to_owned()))
            });
        },());
    }
    html!{<NavBar username={username.deref().clone()}/>}
}

#[function_component]
fn NavBar(authwidgetprops: &AuthWidgetProps) -> Html {
    html! {
        <nav class="navbar navbar-expand-lg bg-body-tertiary">
            <div class="container-fluid">
                <NavBrand name="RW Engineering" />
                <NavToggler />
                <Navigator authwidgetprops={authwidgetprops.clone()}>
                    <NavItem to={Route::Index} text="Home" active={true} />
                    <NavItem to={Route::PeopleRoot} text="People" />
                    <NavItem to={Route::ProjectsRoot} text="Projects" />
                </Navigator>
            </div>
        </nav>
    }
}

#[derive(Properties, PartialEq)]
struct NavItemProps {
    pub to: Route,
    pub text: String,
    #[prop_or_default()]
    pub active: bool,
}

#[function_component]
fn NavItem(NavItemProps { to, text, active }: &NavItemProps) -> Html {
    let mut classes = vec!["nav-link"];
    if *active == true {
        classes.push("active");
    }
    html! {
        <li class="nav-item">
                <Link<Route> classes={classes!{classes.clone()}} to={to.clone()}>{ text.clone() }</Link<Route>>
        </li>
    }
}


#[derive(Properties, PartialEq)]
struct NavBrandProps{
    name: String
}

#[function_component]
fn NavBrand(NavBrandProps { name }: &NavBrandProps) -> Html {
    html! {
        <Link<Route> classes="navbar-brand" to={Route::Index}>
            {name}
        </Link<Route>>
    }
}

#[function_component]
fn NavToggler() -> Html {
    html! {
        <button class="navbar-toggler"
            type="button"
            data-bs-toggle="collapse"
            data-bs-target="#navbarSupportedContent"
                  aria-controls="navbarSupportedContent"
                  aria-expanded="false"
                  aria-label="Toggle navigation">
            <span class="navbar-toggler-icon"></span>
        </button>
    }
}


#[derive(Properties, PartialEq, Clone)]
struct NavigatorProps{
    children: Children,
    authwidgetprops:AuthWidgetProps
}

#[function_component]
fn Navigator(NavigatorProps { children, authwidgetprops }: &NavigatorProps) -> Html{
    html!{
        <div class="collapse navbar-collapse" id="navbarSupportedContent">
            <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                { children.clone() }
            </ul>
            <AuthWidget ..authwidgetprops.clone()/>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct AuthWidgetProps{
    pub username: Option<String>
}

#[function_component]
fn AuthWidget(AuthWidgetProps { username }: &AuthWidgetProps) -> Html {

    match username{
        None => html!{ 
            <>
                <div class="nav-item d-xl-inline-flex">
                    <p class="m-0 pe-2">{"Do not have an account? "}
                        <Link<AuthRoute> to={AuthRoute::Signup}>{"Sign up"}</Link<AuthRoute>>
                    </p>
                </div>
                     <Link<AuthRoute> classes={classes!{"btn", "btn-primary"}} to={AuthRoute::Login}>{"Log in"}</Link<AuthRoute>>
            </>
        },
        Some(username) => html!{
            <>
            <p class="m-0 p-1">{"Logged in as"}</p>
            <div class="dropdown">
                <a class="nav-link dropdown-toggle fw-bold me-3"
                       role="button"
                       data-bs-toggle="dropdown"
                       aria-expanded="false">{username}</a>
                <ul class="dropdown-menu dropdown-menu-lg-end">
                    <li>
                        <span class="dropdown-header">
                            {"Account"}
                        </span>
                    </li>
                    <li>
                        <Link<AuthRoute> classes={classes!{"dropdown-item"}} to={AuthRoute::Profile}>{"Profile"}</Link<AuthRoute>>
                    </li>
                    <li class="dropdown-divider"></li>
                    <li>
                        <Link<AuthRoute> classes={classes!{"dropdown-item"}} to={AuthRoute::Logout}>{"Log Out"}</Link<AuthRoute>>
                    </li>
                </ul>
            </div>
            </>
        } 
    }
}

