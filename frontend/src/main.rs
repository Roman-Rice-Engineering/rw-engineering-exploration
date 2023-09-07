mod navbar;
use navbar::NavBar;

use yew::prelude::*;

#[function_component]
fn App() -> Html{

    html!{
        <>
        <NavBar/>
        {"Hello world!"}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
