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
    ProjectsRoot,
    #[at("/projects/*")]
    Projects,

    #[at("/people")]
    PeopleRoot,
    #[at("/people/*")]
    People,

    #[at("/blog")]
    BlogRoot,
    #[at("/blog/*")]
    Blog,

    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum BlogRoute{
    #[at("/blog/create")]
    CreateBlog,

    #[at("/blog/view/:uuid")]
    ViewBlog{ uuid: String },

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

    #[at("/auth/signup")]
    Signup,
    
    #[not_found]
    #[at("/auth/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum PeopleRoute{
    #[at("/people")]
    Index,

    #[at("/people/:uuid")]
    Person{ uuid: String},

    #[not_found]
    #[at("/auth/404")]
    NotFound,
}
