use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "fridge_product_joins")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// 냉장고 ID
    pub fridge_id: i64,
    /// 제품 ID
    pub product_id: i64,
    /// 유통기한
    pub expiry: Option<DateTime>,
    /// 구입날짜
    pub purchase_date: Option<DateTime>
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::fridge::Entity",
        from = "Column::FridgeId",
        to = "super::fridge::Column::Id"
    )]
    Fridge,
    #[sea_orm(
        belongs_to = "super::product::Entity",
        from = "Column::ProductId",
        to = "super::product::Column::Id"
    )]
    Product
}

impl Related<super::fridge::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Fridge.def()
    }
}

impl Related<super::product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}


impl ActiveModelBehavior for ActiveModel {}