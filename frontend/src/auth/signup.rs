use yew::{function_component, Html, html};


#[function_component]
pub fn Signup() -> Html {
   
    html!{
        <section style="height: 90vh;" class="bg-caution">
            <div class="container h-100">
              <div class="row d-flex justify-content-center align-items-center h-100">
                <div class="col-12 col-md-8 col-lg-6 col-xl-5">
                  <div class="card" style="border-radius: 25px;">
                    <div class="card-body p-5">
                      <h2 class="text-center mb-5">{"Create an account"}</h2>
                      <form>
                        <FormTextInput />
                        <FormTextInput />
                        <FormTextInput />
                        <FormTextInput />
                        <FormSubmitButton />
                        <p class="text-center text-muted mt-4">{"Already have an account? "}<a class="fw-bold">{"Log in"}</a></p>
                      </form>
                    </div>
                  </div>
                </div>
              </div>
            </div>
        </section>
    }
}

#[function_component]
fn FormTextInput() -> Html {
    html!{
        <div class="form-outline mb-4">
            <input type="text" class="form-control form-control-lg" />
        </div>

    }
}

#[function_component]
fn FormSubmitButton() -> Html{
    html!{
        <div class="d-flex justify-content-center">
            <button type="button" class="text-body btn btn-success btn-lg">{"Submit form"}</button>
        </div>

    }
}
