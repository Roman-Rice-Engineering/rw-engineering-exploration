use common::models::base64_files::Base64File;
use markdown::to_html_with_options;
use regex::{Regex, Captures};
use yew::{Properties, function_component, AttrValue, Html, html};

#[derive(PartialEq, Clone, Properties)]
pub struct BlogPostProps{
    pub markdown: String,
    pub files: Vec<Base64File>
}

#[function_component]
pub fn BlogPost( BlogPostProps { markdown, files}: &BlogPostProps ) -> Html{
    
    // Parse the markdown to html string
    let mut markdown: String = match to_html_with_options(&markdown, &markdown::Options::gfm()){
        Err(e) => format!("Error: {e}"),
        Ok(c)=> c
    };

    // Do custom string parsing to allow for special image syntax
    {
        let regex = Regex::new(r#"<img src="(\d+)" alt(.*)>"#).unwrap();
        let result = regex.replace_all(&markdown, |caps: &Captures| {
            let index = caps[1].parse::<usize>().unwrap();
            let alt = &caps[2];
            format!("<img src=\"data:image/png;base64,{}\" alt{}>", match files.get(index){
                Some(c) => c.get_data_base64_str(),
                None => ""
            }, alt)
        }).to_string();
        markdown = result;
    }

    // Render html in rust DOM format from string format
    let markdown: Html = Html::from_html_unchecked(AttrValue::from(markdown));

    html!{
        <div class="w-100 markdown-body p-4">
            {markdown}
        </div>
    }
}

