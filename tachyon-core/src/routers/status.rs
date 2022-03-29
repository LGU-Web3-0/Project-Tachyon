use crate::State;
use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::web::Json;
use actix_web::Result;
use entity::sea_orm::{EntityTrait, PaginatorTrait};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Status {
    kv_debug: String,
    sql_debug: String,
    obj_count: usize,
}

pub async fn handler(data: web::Data<State>) -> Result<Json<Status>> {
    Ok(Json(Status {
        kv_debug: format!("{:#?}", data.kv_db),
        sql_debug: format!("{:#?}", data.sql_db),
        obj_count: entity::object::Entity::find()
            .count(&data.sql_db)
            .await
            .map_err(|e| {
                actix_web::error::InternalError::new(
                    e.to_string(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            })?,
    }))
}
