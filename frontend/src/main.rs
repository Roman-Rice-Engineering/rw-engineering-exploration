mod navbar;
use navbar::MainNav;

mod route;
use route::Route;

use yew::prelude::*;
use yew_router::prelude::*;

fn switch(route: Route) -> Html{
    let body = match route{
        Route::Index => html!{"Index Page!"},
        Route::Projects => html!{"Projects Page!"},
        Route::Auth => html!{"Auth Page!"},
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
