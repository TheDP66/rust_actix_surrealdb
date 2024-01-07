use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Deserialize, Serialize)]
pub struct BuyPizzaRequest {
    #[validate(length(min = 1, message = "pizza name reuired"))]
    pub pizza_name: String,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct UpdatePizzaUrl {
    pub uuid: String,
}

#[derive(Validate, Deserialize, Serialize, Debug)]
pub struct Pizza {
    pub uuid: String,
    pub pizza_name: String,
}

impl Pizza {
    pub fn new(uuid: String, pizza_name: String) -> Pizza {
        Pizza { uuid, pizza_name }
    }
}
