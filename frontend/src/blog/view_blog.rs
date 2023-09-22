use std::ops::Deref;

use common::models::{blog::Blog, base64_files::Base64File};
use gloo_net::http::Request;
use yew::{function_component, Html, html, Properties, use_state, use_effect_with_deps};

use crate::{util::api_request::api_request, env::{PUBLIC_CLOUD_STORAGE, BUCKET_NAME}};

#[derive(Clone, PartialEq, Properties)]
pub struct ViewBlogProps{
    pub uuid: String
}

#[function_component]
pub fn ViewBlog(ViewBlogProps { uuid }: &ViewBlogProps) -> Html{

    let uuid = uuid.to_owned();
    let blog_post = use_state(|| Option::<Base64File>::default());
    let blog_post_cloned = blog_post.clone();
    use_effect_with_deps(move |_| {
       let blog_post_cloned = blog_post_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move{
            let response = api_request(&("/blog/get/".to_owned() + &uuid), None).await;
            let response: Base64File = match response {
                Err(_) => return,
                Ok(c) => match serde_json::from_str(&c){
                    Err(_) => return,
                    Ok(c) => c
                }
            };
            blog_post_cloned.set(Some(response));
        })
    }, ());

    let blog_data = match &*blog_post{
        Some(blog_post) => String::from_utf8(blog_post.get_data_vec_u8().unwrap()).unwrap(),
        None => "".to_owned()
    };

    html!{
        <div class="container">
            <crate::blog::BlogPost markdown={blog_data} files={Vec::new()}/>
        </div>
    }
}


