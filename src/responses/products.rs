use rocket_okapi::JsonSchema;
use serde::Serialize;
use entity::product;

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct ProductResponse {
    pub data: product::Model,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct ProductList {
    pub data: Vec<FridgeProduct>,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct FridgeProduct {
    pub id: i64,
    pub name: String,
    pub category: Option<String>,
    pub expiry: Option<String>,
    pub purchase_date: Option<String>,
}