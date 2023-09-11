use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,

    #[at("/auth")]
    AuthRoot,
    #[at("/auth/*")]
    Auth,

    #[at("/projects")]
    Projects,

    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum AuthRoute{
    #[at("/auth/login")]
    Login,

    #[at("/auth/logout")]
    Logout,

    #[at("/auth/profile")]
    Profile,
    
    #[not_found]
    #[at("/auth/404")]
    NotFound,
}
