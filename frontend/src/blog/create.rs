use std::ops::Deref;

use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlTextAreaElement};
use yew::{function_component, Html, html, use_state, Callback, AttrValue};
use markdown::to_html;


#[function_component]
pub fn CreateBlog() -> Html{
    
    html!{
        <div class="container">
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
    let markdown_change = Callback::from(move |event: Event| {
        let markdown = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlTextAreaElement>()
            .value();
        rendered_markdown_cloned.set(to_html(&markdown));
        plain_markdown_cloned.set(markdown);

    });
    let markdown_html = Html::from_html_unchecked(AttrValue::from(rendered_markdown.deref().clone()));

    html!{
        <div class="row h-100">
            <div class="col">
                <textarea onchange={markdown_change} class="w-100" rows="100"/>
            </div>
            <div class="col">
                <html class="w-100 h-100">
                    {markdown_html}
                </html>
            </div>
        </div>
    }
}
