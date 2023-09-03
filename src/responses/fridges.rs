use rocket_okapi::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct FridgeList {
    pub data: Vec<FridgeData>,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct FridgeData {
    pub id: i64,
    pub name: String,
}
