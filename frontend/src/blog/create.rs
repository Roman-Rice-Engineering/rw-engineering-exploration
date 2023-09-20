use std::ops::Deref;

use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlTextAreaElement, HtmlInputElement, FileList, InputEvent};
use yew::{function_component, Html, html, use_state, Callback, AttrValue};
use markdown::to_html_with_options;


#[function_component]
pub fn CreateBlog() -> Html{

    let file = use_state(|| Option::<FileList>::default());
    let file_cloned = file.clone();
    let file_change = Callback::from(move |event: Event| {
        let file_val = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .files();
        file_cloned.set(file_val.clone());
        log!(file_val);
    });

    html!{
        <div class="container">
            <input type="file" onchange={file_change} />
            <MarkdownEditor />
        </div>
    }
}

#[function_component]
fn MarkdownEditor() -> Html{
    let plain_markdown = use_state(|| String::new());
    let rendered_markdown = use_state(|| String::new());

    let plain_markdown_cloned = plain_markdown.clone();
    let rendered_markdown_cloned = rendered_markdown.clone();
    let markdown_change = Callback::from(move |event: InputEvent| {
        let markdown = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlTextAreaElement>()
            .value();
        rendered_markdown_cloned.set(match to_html_with_options(&markdown, &markdown::Options::gfm()){
            Err(e) => format!("Error: {e}"),
            Ok(c)=> c
        });
        plain_markdown_cloned.set(markdown);

    });
    let markdown_html = Html::from_html_unchecked(AttrValue::from(rendered_markdown.deref().clone()));

    html!{
        <div class="row h-100">
            <div class="col">
                <textarea oninput={markdown_change} class="w-100" rows="100"/>
            </div>
            <div class="col">
                <html id="markdown" class="w-100 h-100">
                    <style>{"#markdown body{font-family:'Helvetica Neue',sans;background:#fff;color:#222;margin:2em auto;padding:0 2em;width:48em;line-height:1.5em;font-size:16px;font-weight:300}#markdown img{margin:0;border:0}#markdown p{margin:1em 0}#markdown a{color:#00213D}#markdown a:visited{color:#00213D;background-color:transparent}#markdown a:active{color:#318100;background-color:transparent}#markdown a:hover{text-decoration:none}#markdown p img{border:0;margin:0}#markdown h1,h2,h3,h4,h5,h6{color:#003a6b;background-color:transparent;margin:1em 0;font-weight:400}#markdown h1{font-size:180%}#markdown h2{font-size:160%}#markdown h3{font-size:140%}#markdown h4{font-size:110%}#markdown h5{font-size:105%}#markdown h6{font-size:100%}#markdown dt{font-style:italic}#markdown dd{margin-bottom:1.5em}#markdown li{line-height:1.5em}#markdown code{padding:.1em;font-size:14px;font-family:'Menlo',monospace;background-color:#f5f5f5;border:1px solid #efefef}#markdown pre{font-family:'Menlo',monospace;background-color:#fff;padding:.5em;line-height:1.25em;border:1px solid #efefef;border-bottom:1px solid #ddd;-webkit-box-shadow:0 1px 3px 0 #eee;-moz-box-shadow:0 1px 3px 0 #eee;-ms-box-shadow:0 1px 3px 0 #eee;box-shadow:0 1px 3px 0 #eee}#markdown pre code{background-color:transparent;border-width:0}#markdown blockquote{border-top:1px solid #efefef;border-bottom:1px solid #ddd;-webkit-box-shadow:0 1px 3px 0 #eee;-moz-box-shadow:0 1px 3px 0 #eee;-ms-box-shadow:0 1px 3px 0 #eee;box-shadow:0 1px 3px 0 #eee}#markdown table{border-collapse:collapse;border:1px solid #efefef;border-bottom:1px solid #ddd;-webkit-box-shadow:0 1px 3px 0 #eee;-moz-box-shadow:0 1px 3px 0 #eee;-ms-box-shadow:0 1px 3px 0 #eee;box-shadow:0 1px 3px 0 #eee}#markdown td,th{border:1px solid #ddd;padding:.5em}#markdown th{background-color:#f5f5f5}"}</style>
                    {markdown_html}
                </html>
            </div>
        </div>
    }
}
