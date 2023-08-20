use serde::{Deserialize, Serialize};
use yewdux::prelude::Store;

#[derive(Default, Clone, PartialEq, Eq, Store, Serialize, Deserialize, Debug)]
#[store(storage = "local")]
pub(crate) struct AppState {
    pub(crate) bearer_token: String,
}
