mod navbar;
use navbar::MainNav;

mod route;
use route::{Route, AuthRoute};

use yew::prelude::*;
use yew_router::prelude::*;

fn switch_auth(route: AuthRoute) -> Html {
    match route{
        AuthRoute::Login => html!{"Login Page!"},
        AuthRoute::Logout => html!{"Logout Page!"},
        AuthRoute::Profile => html!{"Your Profile!"},
        AuthRoute::NotFound => html!{<Redirect<Route> to={Route::NotFound}/>}
    }
}

fn switch(route: Route) -> Html{
    let body = match route{
        Route::Index => html!{"Index Page!"},
        Route::Projects => html!{"Projects Page!"},
        Route::Auth | Route::AuthRoot => html!{<Switch<AuthRoute> render={switch_auth}/>},
        Route::NotFound => html!{"Error 404"}
    };
    html!{
        <>
        <MainNav username="Cool"/>
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
