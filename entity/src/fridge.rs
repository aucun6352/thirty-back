use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize, FromForm)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "product")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// 냉장고 이름
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::fridge_product_join::Entity")]
    FridgeProductJoin
}

impl Related<super::fridge_product_join::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FridgeProductJoin.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
