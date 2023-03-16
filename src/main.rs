#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Serialize};

use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{openapi, openapi_get_routes, JsonSchema};

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct Response {
    test: String,
}

#[openapi]
#[get("/")]
fn index() -> Json<Response> {
    Json(Response {
        test: "test".to_string(),
    })
}





#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", openapi_get_routes![index])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}
