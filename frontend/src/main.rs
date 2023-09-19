mod navbar;
use navbar::MainNav;
mod util;

mod auth;
mod people;

mod env;

mod route;
use route::{Route, AuthRoute, PeopleRoute};

use yew::prelude::*;
use yew_router::prelude::*;

fn switch_auth(route: AuthRoute) -> Html {
    match route{
        AuthRoute::Login => html!{<auth::Login />},
        AuthRoute::Logout => html!{<auth::Logout />},
        AuthRoute::Profile => html!{"Your Profile!"},
        AuthRoute::Signup => html!{<auth::Signup />},
        AuthRoute::NotFound => html!{<Redirect<Route> to={Route::NotFound}/>}
    }
}

fn switch_people(route: PeopleRoute) -> Html {
    match route {
        PeopleRoute::Index => html!{<people::People />},
        PeopleRoute::Person{ uuid } => html!{<people::PeoplePerson uuid={uuid}/>},
        PeopleRoute::NotFound => html!{<Redirect<Route> to={Route::NotFound}/>}
    }
}

fn switch(route: Route) -> Html{
    let body = match route{
        Route::Index => html!{"Index Page!"},
        Route::Projects | Route::ProjectsRoot => html!{"Projects Page!"},
        Route::People | Route::PeopleRoot => html!{<Switch<PeopleRoute> render={switch_people}/>},
        Route::Auth | Route::AuthRoot => html!{<Switch<AuthRoute> render={switch_auth}/>},
        Route::NotFound => html!{"Error 404"}
    };
    html!{
        <>
        <MainNav />
        {body}
        </>
    }
}

#[function_component]
fn App() -> Html {
    html!{
        <BrowserRouter>
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
