mod navbar;
use navbar::MainNav;
mod util;

mod auth;
mod people;
mod blog;

mod env;

mod route;
use route::{Route, AuthRoute, PeopleRoute, BlogRoute};

use yew::prelude::*;
use yew_router::prelude::*;

fn switch_auth(route: AuthRoute) -> Html {
    match route{
        AuthRoute::Login => html!{<auth::Login />},
        AuthRoute::Logout => html!{<auth::Logout />},
        AuthRoute::Profile => html!{<auth::Profile />},
        AuthRoute::Signup => html!{<auth::Signup />},
        AuthRoute::NotFound => html!{<Redirect<Route> to={Route::NotFound}/>}
    }
}

fn switch_blog(route: BlogRoute) -> Html {
    match route{
        BlogRoute::CreateBlog => html!{<blog::create::CreateBlog />},
        BlogRoute::ViewBlog{ uuid } => html!{<>{"Viewing blog: "}{uuid}</>},
        BlogRoute::NotFound => html!{<Redirect<Route> to={Route::NotFound}/>}
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
        Route::Blog | Route::BlogRoot => html!{<Switch<BlogRoute> render={switch_blog}/>},
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
