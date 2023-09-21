use std::ops::Deref;
use common::models::base64_files::Base64File;
use common::models::blog;
use gloo::console::log;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Event, HtmlTextAreaElement, HtmlInputElement, InputEvent, SubmitEvent, MouseEvent};
use yew::{function_component, Html, html, use_state, Callback};
use crate::blog::BlogPost;

use crate::util::api_request::api_request;

use super::BlogPostProps;

#[function_component]
pub fn CreateBlog() -> Html{
    let files = use_state(|| Vec::<Base64File>::new());
    let files_cloned = files.clone();
    let files_change = Callback::from(move |event: Event| {
        let files_cloned = files_cloned.clone();
        let js_files = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .files()
            .unwrap();
        log!(js_files.clone());
        let mut input_files: Vec<Base64File> = Vec::new();
        let closure = wasm_bindgen_futures::spawn_local(async move{
            for i in 0..js_files.length(){
                let file = js_files.item(i).unwrap();
                let file_value: Vec<u8> = js_sys::Uint8Array::new(&JsFuture::from(file.slice().unwrap().array_buffer()).await.unwrap()).to_vec();
                input_files.push(Base64File::new_from_vec_u8(&file_value, file.type_()));
            }
            files_cloned.set(input_files);
        });
    });
    
    let files_cloned = files.clone();
    let view_images: Html = files_cloned.iter().map(|file| base_64_to_html_image(file.get_data_base64_str())).collect(); 
    
    let onsubmit = Callback::from(move |event: SubmitEvent|{
        event.prevent_default();
    });
    html!{
        <div class="container-fluid">
            <div class="p-3">
                <input type="file" multiple={true} onchange={files_change} />
                {view_images.clone()}
            </div>
            <MarkdownEditor markdown={"".to_owned()} files={files.deref().to_vec()}/>
        </div>
    }
}

#[function_component]
fn MarkdownEditor(BlogPostProps { files, ..}: &BlogPostProps) -> Html{
    let plain_markdown = use_state(|| String::new());
    let plain_markdown_cloned = plain_markdown.clone();
    let markdown_change = Callback::from(move |event: InputEvent| {
        let markdown = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlTextAreaElement>()
            .value();
        plain_markdown_cloned.set(markdown);
    });
    let files_cloned = files.clone();
    let plain_markdown_cloned = plain_markdown.clone();
    let onclick = Callback::from(move |event: MouseEvent|{
        let files_cloned = files_cloned.clone();
        let plain_markdown_cloned = plain_markdown_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move{
            api_request("/blog/create/", Some(serde_json::to_string_pretty(
                &common::models::base64_files::BlogPost::new(files_cloned.deref().to_owned(), plain_markdown_cloned.deref().to_owned())
            ).unwrap())).await;
        });
    });


    html!{
        <>
        <button class="btn btn-primary m-3 p-2" {onclick}>{"Submit"}</button>
        <div class="row">
            <div class="col">
                <textarea oninput={markdown_change} class="w-100" style="height: 100vh" />
            </div>
            <div class="col">
                <BlogPost markdown={plain_markdown.deref().clone()} files={files.deref().to_vec()}/>
            </div>
        </div>
        </>
    }
}

fn base_64_to_html_image(image_as_base64: &str) -> Html{
     html!{
         <img style="
         height: 100px;
         width: 100px;
         object-fit: cover;"
         src={
         "data:image/png;base64,".to_owned() + image_as_base64
         }/>}
}
