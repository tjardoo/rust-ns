use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiMessage {
    pub message: String,
    pub style: String,
}
