use rocket_okapi::JsonSchema;
use serde::Serialize;

mod fridges;
pub use fridges::*;

mod products;
pub use products::*;


#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct HealthCheck {
    pub check: String,
}