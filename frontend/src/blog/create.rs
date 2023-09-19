use yew::{function_component, Html, html, use_state};



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
    let markdown = use_state(|| String::new());
    let rendered_markdown = use_state(|| String::new());
    
    html!{
        <div class="row h-100">
            <div class="col">
                <textarea class="w-100" rows="100"/>
            </div>
            <div class="col">
            </div>
        </div>
    }
}
