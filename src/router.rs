use crate::components::LoginMask;
use crate::components::TicketsList;
use crate::models::AppState;
use serde_json::json;
use web_sys::window;
use yew::{html, Html};
use yew_router::Routable;
use crate::components::TicketView;

pub(crate) fn switch(routes: Route) -> Html {
    let json_string = window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap()
        .get("cira_frontend::models::AppState")
        .unwrap()
        .unwrap_or(
            json!({
                "bearer_token": ""
            })
            .to_string(),
        )
        .to_string();
    let app_state = serde_json::from_str::<AppState>(&json_string).unwrap();

    match routes {
        Route::Wrapper => {
            if !app_state.bearer_token.is_empty() {
                return html! { <TicketsList /> };
            }
            html! { <LoginMask /> }
        },
        Route::TicketView { id } => {
            html! { <TicketView ticket_id={id} />}
        }
    }
}

#[derive(Clone, Routable, PartialEq)]
pub(crate) enum Route {
    #[at("/")]
    Wrapper,
    #[at("/tickets/:id")]
    TicketView { id: i32 }
}
