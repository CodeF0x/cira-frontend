use crate::models::{AppState, Status, Ticket, TicketViewProps};
use crate::router::switch;
use crate::router::Route;
use chrono::prelude::DateTime;
use chrono::Local;
use gloo_net::http;
use gloo_net::http::Headers;
use serde_json::json;
use std::time::{Duration, UNIX_EPOCH};
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
            <div class="card">
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

#[function_component]
pub(crate) fn TicketsList() -> Html {
    let (state, _dispatch) = use_store::<AppState>();
    let tickets = use_state_eq(|| vec![]);

    let timestamp_to_date = |timestamp: &str| {
        let d = UNIX_EPOCH
            + Duration::from_millis(timestamp.parse::<i64>().unwrap().try_into().unwrap());
        let datetime = DateTime::<Local>::from(d);
        let timestamp_str = datetime.format("%d.%m.%Y %H:%M").to_string();

        timestamp_str
    };

    {
        let tickets = tickets.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let bearer = &state.bearer_token;
            let headers = Headers::new();
            headers.append("Authorization", &format!("Bearer {}", bearer));

            let fetched_tickets: Vec<Ticket> =
                http::Request::get("http://localhost:8081/api/tickets")
                    .headers(headers)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

            tickets.set(fetched_tickets);
        });
    }

    html! {
        <section class="p-3">
            <table class="table">
                <thead>
                    <tr>
                    <th scope="col">{ "#" }</th>
                    <th scope="col">{ "Title" }</th>
                    <th scope="col">{ "Status" }</th>
                    <th scope="col">{ "Labels" }</th>
                    <th scope="col">{ "Created" }</th>
                    <th scope="col">{ "Last edited" }</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        tickets
                            .iter()
                            .map(|ticket| {
                                html! {
                                    <tr>
                                        <th scope="row">{ &ticket.id }</th>
                                        <td>
                                            <a href={format!("/tickets/{}", &ticket.id)}>{ &ticket.title }</a>
                                        </td>
                                        <td>{ *&ticket.status }</td>
                                        <td>{
                                                ticket
                                                    .labels
                                                    .clone()
                                                    .iter()
                                                    .map(
                                                        |label| html! { <span class="badge bg-secondary mx-1">{ label.to_string() }</span>}
                                                    )
                                                    .collect::<Html>()
                                        }</td>
                                        <td>{ timestamp_to_date(&ticket.created) }</td>
                                        <td>{ timestamp_to_date(&ticket.last_modified) }</td>
                                    </tr>
                                }
                            })
                        .collect::<Html>()
                    }
                </tbody>
            </table>
        </section>
    }
}

#[function_component]
pub(crate) fn TicketView(props: &TicketViewProps) -> Html {
    let (state, _dispatch) = use_store::<AppState>();
    let ticket = use_state_eq(|| Ticket {id: 0, title: "".to_string(), body: "".to_string(), labels: vec![], assigned_user: None, last_modified: "".to_string(), created: "".to_string(), status: Status::Open});
    let id = props.ticket_id;

    {
        let ticket = ticket.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let ticket = ticket.clone();
            let headers = Headers::new();
            headers.append("Authorization", &format!("Bearer {}", state.bearer_token));

            let fetch_ticket: Ticket = http::Request::get(&format!("http://localhost:8081/api/tickets/{}", id)).headers(headers).send().await.unwrap().json().await.unwrap();
            ticket.set(fetch_ticket);
        });
    }


    html! {
        <div class="card">
            <div class="card-body">
                <h5 class="card-title">{ &ticket.title }</h5>
                <h6 class="card-subtitle">{ "Status: " }{ *&ticket.status }</h6>
                <p class="card-text">{ &ticket.body }</p>
                <a class="card-link" href="/">{ "Back" }</a>
            </div>
        </div>
    }
}