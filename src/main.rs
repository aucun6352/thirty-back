#[macro_use]
extern crate rocket;

use entity::prelude::*;
use entity::*;

use rocket::http::Status;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::{
    fairing::{self, AdHoc},
    Build, Rocket,
};

use migration::MigratorTrait;
use rocket_okapi::rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{openapi, openapi_get_routes};
use sea_orm::{entity::*, QueryFilter};
use sea_orm_rocket::{Connection, Database};
use securestore::{KeySource, SecretsManager};
use simple_logger::SimpleLogger;

mod open_api_response;
mod pool;
mod responses;
use pool::Db;

#[openapi]
#[get("/")]
fn index() -> Json<responses::HealthCheck> {
    Json(responses::HealthCheck {
        check: "ok".to_string(),
    })
}

/// # 바코드로 제품정보 알기
///
/// 바코드를 보내주면 제품정보 리턴
#[openapi(tag = "Barcode")]
#[get("/barcode/<barcode>")]
async fn get_barcode(
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

/// # 냉장고에 있는 제품 리스트
///
/// 냉장고에 있는 제품 리스트
#[openapi(tag = "Fridge")]
#[get("/fridges/<id>/products")]
async fn get_product_list(
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
#[openapi(tag = "Fridge")]
#[post("/fridges/<id>/products/<product_id>")]
async fn input_product(
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
#[openapi(tag = "Fridge")]
#[delete("/fridges/<id>/products/<fridge_product_join_id>")]
async fn delete_product(
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

/// # 냉장고 리스트
#[openapi(tag = "Fridge")]
#[get("/fridges")]
async fn list_fridges(conn: Connection<'_, Db>) -> Option<Json<responses::FridgeList>> {
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

async fn get_barcode_info(
    db: &sea_orm::DatabaseConnection,
    barcode: &String,
) -> Option<product::ActiveModel> {
    let api_key = secrets_manager().get("open_api::api_key").unwrap();
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

fn secrets_manager() -> SecretsManager {
    SecretsManager::load("secrets.json", KeySource::File("secrets.key")).unwrap()
}

#[launch]
fn rocket() -> _ {
    SimpleLogger::new().init().unwrap();

    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount(
            "/",
            openapi_get_routes![
                index,
                get_barcode,
                input_product,
                delete_product,
                list_fridges,
                get_product_list
            ],
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}
