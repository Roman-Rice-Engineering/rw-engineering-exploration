use yew::prelude::*;

#[function_component]
pub fn NavBar() -> Html {
    html! {
        <>
            <nav class="navbar navbar-expand-lg bg-body-tertiary">
      <div class="container-fluid">
        <a class="navbar-brand" href="#">
        {"RW Engineering"}              //Change Later
        </a>
        <button class="navbar-toggler"
                type="button"
                data-bs-toggle="collapse"
                data-bs-target="#navbarSupportedContent"
                aria-controls="navbarSupportedContent"
                aria-expanded="false"
                aria-label="Toggle navigation">
          <span class="navbar-toggler-icon"></span>
        </button>
        <div class="collapse navbar-collapse" id="navbarSupportedContent">
          <ul class="navbar-nav me-auto mb-2 mb-lg-0">
            <NavItem href="/" text="Home" active={true}/>
            <NavItem href="/about/" text="About" />
            <NavItem href="/projects/" text="Projects" />
            <NavItem href="/blog/" text="Blog" />
              </ul>
              <div class="nav-item d-inline-flex py-2 me-3">
                <i class="bi bi-brightness-high-fill"></i>
                <div class="form-check form-switch mx-3" style="transform: scale(1.5)">
                  <input /*onclick="const root = document.querySelector(':root'); if(root.getAttribute('data-bs-theme') == 'light'){ root.setAttribute('data-bs-theme', 'dark'); }else{ root.setAttribute('data-bs-theme', 'light'); }"*/
                         class="form-check-input"
                         type="checkbox"
                         role="switch"
                         id="flexSwitchCheckDefault"/>
                </div>
                <i class="bi bi-moon-stars nav-item e-auto ps-0"
                   style="transform: translate3d(-12.5px, 0px, 0px)"></i>
              </div>
              <div class="nav-item d-xl-inline-flex">
                <p class="m-0 pe-2">{"Dont have an account? "}<a href="/accounts/signup/">{"Sign up"}</a></p>
                </div>
                  <a href="/accounts/login/" class="btn btn-primary">{"Log in"}</a>
              </div>
            </div>
        </nav>
        </>
    }
}


#[derive(Properties, PartialEq)]
struct NavItemProps {
    pub href: String,
    pub text: String,
    #[prop_or_default()]
    pub active: bool
}

#[function_component]
fn NavItem(NavItemProps { href, text, active }: &NavItemProps) -> Html {
        let mut classes = vec!["nav-link"];
        if *active == true{
            classes.push("active");
        }
    html!{
        <li class="nav-item">
            <a class={classes!(classes)} href={ href.clone() }>{ text.clone() }</a>
        </li>
    }   
}
