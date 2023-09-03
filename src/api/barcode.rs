use entity::prelude::*;
use entity::*;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use sea_orm::{entity::*, QueryFilter};
use sea_orm_rocket::Connection;

#[path = "../pool.rs"]
mod pool;
#[path = "../responses/mod.rs"]
mod responses;
use pool::Db;

#[path = "../responses/open_api_response.rs"]
mod open_api_response;

#[path = "../config/server.rs"]
mod server;


/// # 바코드로 제품정보 알기
///
/// 바코드를 보내주면 제품정보 리턴
#[openapi(tag = "Barcode")]
#[get("/barcode/<barcode>")]
pub async fn get_barcode(
    conn: Connection<'_, Db>,
    barcode: String,
) -> Option<Json<responses::ProductResponse>> {
    let db: &sea_orm::DatabaseConnection = conn.into_inner();

    let product_result = Product::find()
        .filter(product::Column::Barcode.eq(&barcode))
        .one(db)
        .await
        .unwrap();

    let data: product::Model = match product_result {
        Some(product) => product,
        None => match get_barcode_info(db, &barcode).await {
            Some(product) => product.try_into_model().unwrap(),
            None => return None,
        },
    };

    Some(Json(responses::ProductResponse { data: data }))
}

async fn get_barcode_info(
    db: &sea_orm::DatabaseConnection,
    barcode: &String,
) -> Option<product::ActiveModel> {
    let api_key = server::secrets_manager().get("open_api::api_key").unwrap();
    let service_code = "C005";
    let response_type = "json";

    let response = reqwest::get(format!(
        "http://openapi.foodsafetykorea.go.kr/api/{}/{}/{}/1/1/BAR_CD={}",
        api_key, service_code, response_type, barcode
    ))
    .await
    .unwrap()
    .json::<open_api_response::C005>()
    .await
    .unwrap();

    log::debug!("get_barcode_info: {:?}", response);

    match response.C005.row {
        None => None,
        Some(mut row) => {
            let data = row.pop().unwrap();
            let mut product: product::ActiveModel = product::ActiveModel {
                name: Set(data.PRDLST_NM),
                category: Set(Some(data.PRDLST_DCNM)),
                barcode: Set(data.BAR_CD),
                day_count: Set(Some(data.POG_DAYCNT)),
                ..Default::default()
            };
            product = product.save(db).await.unwrap();

            return Some(product);
        }
    }
}
