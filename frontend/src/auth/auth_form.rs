use web_sys::{HtmlInputElement, Event};
use yew::{Properties, function_component, Html, html, Children, Callback};
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq, Clone)]
pub struct AuthFormProps{
    pub children: Children,
    pub alert: Html
}

#[function_component]
pub fn AuthForm(AuthFormProps { children, alert }: &AuthFormProps) -> Html {

    html!{
        <section style="height: 90vh;" class="bg-caution">
            <div class="container h-100">
              <div class="row d-flex justify-content-center align-items-center h-100">
                <div class="col-12 col-md-8 col-lg-6 col-xl-5">
                  <div class="card" style="border-radius: 25px;">
                    <div class="card-body p-5">
                        {alert.clone()}
                        <h2 class="text-center mb-5">{"Create an account"}</h2>
                        {children.clone()}
                    </div>
                  </div>
                </div>
              </div>
            </div>
        </section>


    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct FormTextInputProps {
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or("text".to_owned())]
    pub input_type: String,
    pub name: String,
    pub onchange: Callback<String>,
    pub value: String,
}

#[function_component]
pub fn FormTextInput(
    FormTextInputProps {
        value,
        placeholder,
        name,
        input_type,
        onchange,
    }: &FormTextInputProps,
) -> Html {
    let onchange = onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        onchange.emit(value);
    });
    html! {
        <div class="form-outline mb-4">
            <input value={value.clone()} {onchange} type={input_type.clone()} placeholder={placeholder.clone()} name={name.clone()} class="form-control form-control-lg" />
        </div>

    }
}

#[function_component]
pub fn FormSubmitButton() -> Html {
    html! {
            <div class="d-flex justify-content-center">
                <button type="submit" class="text-body btn btn-success btn-lg">{"Submit form"}</button>
            </div>

    }
}
