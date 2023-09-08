mod navbar;
use navbar::MainNav;

use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
        <MainNav/>
        {"Hello world!"}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
