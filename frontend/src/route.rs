use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,

    #[at("/auth")]
    Auth,

    #[at("/projects")]
    Projects,

    #[at("/404")]
    NotFound,
}
