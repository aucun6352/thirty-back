use entity::prelude::*;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use sea_orm::entity::*;
use sea_orm_rocket::Connection;

#[path = "../pool.rs"]
mod pool;
#[path = "../responses/mod.rs"]
mod responses;
use pool::Db;

/// # 냉장고 리스트
#[openapi(tag = "Fridge")]
#[get("/fridges")]
pub async fn list_fridges(conn: Connection<'_, Db>) -> Option<Json<responses::FridgeList>> {
    let db: &sea_orm::DatabaseConnection = conn.into_inner();

    let fridge_record = Fridge::find().all(db).await.unwrap();

    Some(Json(responses::FridgeList {
        data: fridge_record
            .iter()
            .map(|record| responses::FridgeData {
                id: record.id,
                name: record.clone().name,
            })
            .collect(),
    }))
}
