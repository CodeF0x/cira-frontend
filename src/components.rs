use crate::models::AppState;
use crate::router::switch;
use crate::router::Route;
use gloo_net::http;
use serde_json::json;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlDocument, HtmlInputElement};
use yew::prelude::*;
use yew_bootstrap::component::*;
use yew_bootstrap::util::include_cdn;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component]
pub(crate) fn App() -> Html {
    html! {
        <>
            { include_cdn() }
            <Wrapper />
        </>
    }
}

#[function_component]
pub(crate) fn Wrapper() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[function_component]
pub(crate) fn LoginMask() -> Html {
    let (_state, dispatch) = use_store::<AppState>();
    let is_error = use_state(|| false);
    let is_login_error = use_state(|| false);

    let onsubmit = {
        let is_error = is_error.clone();
        let is_login_error = is_login_error.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let is_error = is_error.clone();
            let is_login_error = is_login_error.clone();
            let dispatch = dispatch.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let dispatch = dispatch.clone();
                let document = window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .dyn_into::<HtmlDocument>()
                    .unwrap();

                let password = document
                    .get_element_by_id("password")
                    .unwrap()
                    .dyn_into::<HtmlInputElement>()
                    .unwrap()
                    .value();
                let email = document
                    .get_element_by_id("email")
                    .unwrap()
                    .dyn_into::<HtmlInputElement>()
                    .unwrap()
                    .value();

                if let Ok(response) = http::Request::post("http://localhost:8081/api/login")
                    .json(&json!({"email": email, "password": password}))
                    .unwrap()
                    .send()
                    .await
                {
                    let token = response.text().await.unwrap();
                    let status_code = response.status();
                    if status_code == 200 {
                        dispatch.reduce_mut(|state| state.bearer_token = token);
                        window().unwrap().location().reload().unwrap();
                    } else if status_code == 401 {
                        is_login_error.set(true);
                    }
                } else {
                    is_error.set(true);
                }
            });
        })
    };

    html! {
        <section class="h-100 d-flex align-items-center justify-content-center flex-column gap-3">
            <noscript>
                <div class="alert alert-danger" role="danger">
                    { "This site requires JavaScript to work. Please enable it." }
                </div>
            </noscript>
            <div class="card login-card">
                <div class="card-body">
                    <form {onsubmit}>
                        <div class="card-title">{ "Login to Cira" }</div>
                        <input id="email" class="form-control" type="email" placeholder="Email" required={true} />
                        <br />
                        <input id="password" class="form-control" type="password" placeholder="Password" required={true} />
                        <br />
                        <br />
                        <Button class="btn-primary"> { "Login" }</Button>
                    </form>
                </div>
            </div>

            if *is_error {
                <div class="alert alert-danger" role="alert">
                    { "An error occurred while logging you in." }
                </div>
            } else if *is_login_error {
                <div class="alert alert-danger" role="alert">
                    { "Wrong email or password." }
                </div>
            }

        </section>
    }
}
