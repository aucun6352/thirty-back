use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize, FromForm)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "fridge_products")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub product_id: i64,
    pub expiry: String,
    pub purchase_date: TimeDateTime
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}