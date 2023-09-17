
pub fn auth_reload_login(){
    let _ = match web_sys::window(){
                Some(c) => c.location().set_href("/auth/profile"),
                None => return
    };
}

pub fn auth_reload_logout(){
    let _ = match web_sys::window(){
                Some(c) => c.location().set_href("/"),
                None => return
    };
}
