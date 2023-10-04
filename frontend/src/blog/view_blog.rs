use std::ops::Deref;

use common::models::{blog::Blog, base64_files::{Base64File, BlogPost}};
use gloo_net::http::Request;
use yew::{function_component, Html, html, Properties, use_state, use_effect_with_deps};

use crate::util::api_request::api_request;

#[derive(Clone, PartialEq, Properties)]
pub struct ViewBlogProps{
    pub uuid: String
}

#[function_component]
pub fn ViewBlog(ViewBlogProps { uuid }: &ViewBlogProps) -> Html{

    let uuid = uuid.to_owned();
    let blog_post = use_state(|| Option::<BlogPost>::default());
    let blog_post_cloned = blog_post.clone();
    use_effect_with_deps(move |_| {
       let blog_post_cloned = blog_post_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move{
            let response = api_request(&("/blog/get/".to_owned() + &uuid), None).await;
            let response: BlogPost = match response {
                Err(_) => return,
                Ok(c) => match serde_json::from_str(&c){
                    Err(_) => return,
                    Ok(c) => c
                }
            };
            blog_post_cloned.set(Some(response));
        })
    }, ());

    let markdown = match &*blog_post{
        Some(c) => c.get_markdown().to_owned(),
        None => "".to_owned()
    };

    let files = match &*blog_post{
        Some(c) => c.get_files().to_vec(),
        None => Vec::new()
    };

    html!{
        <div class="container">
            <crate::blog::BlogPost markdown={markdown} {files}/>
        </div>
    }
}


