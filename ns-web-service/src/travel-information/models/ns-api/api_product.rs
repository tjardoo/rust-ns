use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ApiProduct {
    pub number: String,
    pub categoryCode: String,
    pub shortCategoryName: String,
    pub longCategoryName: String,
    pub operatorCode: String,
    pub operatorName: String,
    pub r#type: String,
}
