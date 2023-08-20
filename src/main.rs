use gloo_net::http;
use serde::{Deserialize, Serialize};
use serde_json::json;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlDocument, HtmlInputElement};
use yew::prelude::*;
use yew_bootstrap::component::*;
use yew_bootstrap::util::include_cdn;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Eq, Store, Serialize, Deserialize, Debug)]
#[store(storage = "local")]
struct AppState {
    bearer_token: String,
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Wrapper,
}

#[function_component]
fn App() -> Html {
    html! {
        <>
            { include_cdn() }
            <Wrapper />
        </>
    }
}

#[function_component]
fn Wrapper() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    let json_string = window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap()
        .get("cira_frontend::AppState")
        .unwrap()
        .unwrap_or(
            json!({
                "bearer_token": ""
            })
            .to_string(),
        )
        .to_string();
    let app_state = serde_json::from_str::<AppState>(&json_string).unwrap();

    println!("{:?}", app_state);

    match routes {
        Route::Wrapper => {
            if !app_state.bearer_token.is_empty() {
                return html! { <h1> { "Logged in" } </h1> };
            }
            html! { <LoginMask /> }
        }
    }
}

#[function_component]
fn LoginMask() -> Html {
    let (_state, dispatch) = use_store::<AppState>();
    let is_error = use_state(|| false);

    let onsubmit = {
        let is_error = is_error.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let is_error = is_error.clone();
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

                if let Ok(response) = http::Request::post("http://localhost/login")
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
                    { "An error accoured while logging you in." }
                </div>
            }

        </section>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
