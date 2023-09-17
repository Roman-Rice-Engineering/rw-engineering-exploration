use std::ops::Deref;

use gloo_net::http::Request;

use crate::env::API_URL;


pub async fn api_request(path: &str, body: String) -> Result<String, gloo_net::Error>{
    let post_submission: String = Request::post(&(API_URL.to_owned() + path))
        .header("X-CSRF-Token", match wasm_cookies::get("CSRF_TOKEN"){
            Some(c) => match c {
                Ok(s) => s,
                Err(_) => "".to_owned()
            },
            None => "".to_owned()
        }.deref())
        .body(body)
        .unwrap()
        .send()
        .await?
        .text()
        .await?;
        
    Ok(post_submission)
}
