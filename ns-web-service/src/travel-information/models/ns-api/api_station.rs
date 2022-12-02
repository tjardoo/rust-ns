use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct ApiRouteStation {
    pub uicCode: String,
    pub mediumName: String,
}
