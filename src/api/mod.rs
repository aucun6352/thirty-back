use rocket_okapi::openapi;
use rocket::serde::json::Json;
#[path = "../responses/mod.rs"]
mod responses;

mod fridges;
pub use fridges::*;

mod products;
pub use products::*;

mod barcode;
pub use barcode::*;

#[openapi]
#[get("/")]
pub fn index() -> Json<responses::HealthCheck> {
    Json(responses::HealthCheck {
        check: "ok".to_string(),
    })
}