use std::ops::Deref;
use gloo::console::log;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Event, HtmlTextAreaElement, HtmlInputElement, InputEvent, SubmitEvent};
use yew::{function_component, Html, html, use_state, Callback};
use base64::{engine::general_purpose::STANDARD, Engine};
use crate::blog::BlogPost;

use crate::util::api_request::api_request;

#[function_component]
pub fn CreateBlog() -> Html{
    let files = use_state(|| Vec::<(String, String)>::new());
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
        let mut input_files: Vec<(String, String)> = Vec::new();
        let closure = wasm_bindgen_futures::spawn_local(async move{
            for i in 0..js_files.length(){
                let file = js_files.item(i).unwrap();
                let file_value: Vec<u8> = js_sys::Uint8Array::new(&JsFuture::from(file.slice().unwrap().array_buffer()).await.unwrap()).to_vec();
                let file_value = STANDARD.encode(&file_value);
                input_files.push((file_value, file.type_()));
            }
            files_cloned.set(input_files);
        });
    });
    
    let files_cloned = files.clone();
    let view_images: Html = files_cloned.iter().map(|(data, datatype)| base_64_to_html_image(data)).collect();
    let onsubmit = Callback::from(move |event: SubmitEvent|{
        event.prevent_default();
        let files_cloned = files_cloned.clone(); 
        wasm_bindgen_futures::spawn_local(async move{
            log!(serde_json::to_string_pretty(&files_cloned.deref()).unwrap());
            api_request("/blog/create/", Some(serde_json::to_string_pretty(&files_cloned.deref()).unwrap())).await;
        });
    });
    
    html!{
        <div class="container-fluid">
            <form {onsubmit}>
                <input type="file" multiple={true} onchange={files_change} />
                {view_images.clone()}
                <button type="submit">{"Submit"}</button>
            </form>
            <MarkdownEditor />
        </div>
    }
}

#[function_component]
fn MarkdownEditor() -> Html{
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


    html!{
        <div class="row">
            <div class="col">
                <textarea oninput={markdown_change} class="w-100" style="height: 100vh" />
            </div>
            <div class="col">
                <BlogPost markdown={plain_markdown.deref().clone()} />
            </div>
        </div>
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
