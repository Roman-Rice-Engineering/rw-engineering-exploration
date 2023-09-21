use markdown::to_html_with_options;
use yew::{Properties, function_component, AttrValue, Html, html};

#[derive(PartialEq, Clone, Properties)]
pub struct BlogPostProps{
    pub markdown: String
}

#[function_component]
pub fn BlogPost( BlogPostProps { markdown }: &BlogPostProps ) -> Html{
    
    // Parse the markdown to html string
    let markdown: String = match to_html_with_options(&markdown, &markdown::Options::gfm()){
        Err(e) => format!("Error: {e}"),
        Ok(c)=> c
    };

    // Render html in rust DOM format from string format
    let markdown: Html = Html::from_html_unchecked(AttrValue::from(markdown));

    html!{
        <div class="w-100 markdown-body p-4" style="overflow: scroll;height: 100vh;">
            {markdown}
        </div>
    }
}

