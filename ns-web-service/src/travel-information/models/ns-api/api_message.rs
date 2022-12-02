use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiMessage {
    pub message: String,
    pub style: String,
}
