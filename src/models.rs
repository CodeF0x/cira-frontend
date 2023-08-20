use serde::{Deserialize, Serialize};
use yewdux::prelude::Store;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created: String,
    pub last_modified: String,
    pub labels: Vec<Label>,
    pub assigned_user: Option<i32>,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, Eq)]
pub enum Label {
    Feature,
    Bug,
    WontFix,
    Done,
    InProgress,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy, Eq)]
pub enum Status {
    Open,
    Closed,
}

#[derive(Default, Clone, PartialEq, Eq, Store, Serialize, Deserialize, Debug)]
#[store(storage = "local")]
pub(crate) struct AppState {
    pub(crate) bearer_token: String,
}
