use yew::prelude::*;

#[derive(PartialEq, Properties)]
struct Props {
    children: Html,
}

#[function_component]
fn Container(props: &Props) -> Html {
    html! {
        <div>
            { props.children.clone() }
        </div>
    }
}

#[function_component]
fn tester() -> Html{
html! {
    <Container id="container">
        <h4>{ "Hi" }</h4>
        <div>{ "Hello" }</div>
    </Container>
}}
