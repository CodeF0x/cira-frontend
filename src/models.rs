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

impl ToString for Label {
    fn to_string(&self) -> String {
        match self {
            Label::Feature => "Feature".to_string(),
            Label::Bug => "Bug".to_string(),
            Label::WontFix => "Wont Fix".to_string(),
            Label::Done => "Done".to_string(),
            Label::InProgress => "In Progress".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy, Eq)]
pub enum Status {
    Open,
    Closed,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Closed => "Closed".to_string(),
            Status::Open => "Open".to_string(),
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq, Store, Serialize, Deserialize, Debug)]
#[store(storage = "local")]
pub(crate) struct AppState {
    pub(crate) bearer_token: String,
}
