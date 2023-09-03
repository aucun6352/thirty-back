use entity::prelude::*;
use entity::*;
use rocket::http::Status;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use sea_orm::{entity::*, QueryFilter};
use sea_orm_rocket::Connection;

#[path = "../pool.rs"]
mod pool;
#[path = "../responses/mod.rs"]
mod responses;
use pool::Db;

/// # 냉장고에 있는 제품 리스트
///
/// 냉장고에 있는 제품 리스트
#[openapi(tag = "Product")]
#[get("/fridges/<id>/products")]
pub async fn get_product_list(
    conn: Connection<'_, Db>,
    id: i64,
) -> Option<Json<responses::ProductList>> {
    let db: &sea_orm::DatabaseConnection = conn.into_inner();

    let fridge_record = Fridge::find_by_id(id).one(db).await.unwrap().unwrap();
    let product_relation = fridge_record
        .find_related(FridgeProductJoin)
        .find_also_related(Product)
        .all(db)
        .await
        .unwrap();

    let products = product_relation
        .iter()
        .map(|(product_join, product)| {
            let product_record = product.as_ref().unwrap();
            responses::FridgeProduct {
                id: product_join.id,
                name: product_record.name.clone(),
                category: product_record.category.clone(),
                expiry: Some(product_join.expiry.unwrap_or_default().to_string()),
                purchase_date: Some(product_join.purchase_date.unwrap_or_default().to_string()),
            }
        })
        .collect();

    Some(Json(responses::ProductList { data: products }))
}

/// # 냉장고에 제품 넣기
///
/// 냉장고에 제품 넣기
#[openapi(tag = "Product")]
#[post("/fridges/<id>/products/<product_id>")]
pub async fn input_product(
    conn: Connection<'_, Db>,
    id: i64,
    product_id: i64,
) -> Result<Status, BadRequest<String>> {
    let db: &sea_orm::DatabaseConnection = conn.into_inner();

    let result = fridge_product_join::ActiveModel {
        fridge_id: Set(id),
        product_id: Set(product_id),
        ..Default::default()
    }
    .insert(db)
    .await;

    match result {
        Ok(_) => Ok(Status::Created),
        Err(e) => Err(BadRequest(Some(e.to_string()))),
    }
}

/// # 냉장고에서 제품 삭제하기
#[openapi(tag = "Product")]
#[delete("/fridges/<id>/products/<fridge_product_join_id>")]
pub async fn delete_product(
    conn: Connection<'_, Db>,
    id: i64,
    fridge_product_join_id: i64,
) -> Result<Status, BadRequest<String>> {
    let db: &sea_orm::DatabaseConnection = conn.into_inner();

    let _result = Fridge::find_by_id(id)
        .one(db)
        .await
        .unwrap()
        .unwrap()
        .find_related(FridgeProductJoin)
        .filter(fridge_product_join::Column::Id.eq(fridge_product_join_id))
        .one(db)
        .await
        .unwrap()
        .unwrap()
        .delete(db)
        .await;

    Ok(Status::Created)
}
