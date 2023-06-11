use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use rocket_okapi::JsonSchema;
// use regex::Regex;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize, JsonSchema, Default)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "product")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// # 제품명
    pub name: String,
    /// # 유형
    pub category: Option<String>,
    /// # 바코드
    pub barcode: String,
    /// # 유통기한
    #[schemars(example = "example_day_count")]
    pub day_count: Option<String>,
}

fn example_day_count() -> Vec<String> {
    vec![
        "24개월  실온".to_string(),
        "제조일로부터18개월".to_string(),
        "제조일로부터 12개월까지".to_string(),
        "제조일로부터 12개월".to_string(),
        "제조일로부터 2년까지(실온)".to_string(),
        "30일(냉장)".to_string(),
        "제조일로부터 14일(0~10도씨)".to_string(),
    ]
}

// impl Model {
//     fn expiry(&self) -> chrono::NaiveTime {
//         // self.POG_DAYCNT

//         lazy_static! {
        
//         }

//         Regex
//     }
// }

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
