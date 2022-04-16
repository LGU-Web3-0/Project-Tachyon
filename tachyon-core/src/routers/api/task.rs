use crate::session::UserInfo;
use crate::State;
use actix_session::Session;
use actix_web::error::ErrorInternalServerError;
use actix_web::web::Json;
use actix_web::{http, web, HttpResponse, Result};
use entity::sea_orm::entity::prelude::*;
use entity::sea_orm::{ActiveModelTrait, DatabaseConnection};
use validator::Validate;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AddTaskResult {
    success: bool,
    message: Option<String>,
}

#[derive(PartialEq, serde::Serialize, serde::Deserialize, Debug, Validate)]
pub struct AddTaskRequest {
    #[validate(length(min = 1))]
    name: String,
    create_date: DateTimeUtc,
    due_date: DateTimeUtc,
}

pub async fn add_task(
    request: Json<AddTaskRequest>,
    session: Session,
    data: web::Data<State>,
) -> Result<HttpResponse> {
    async fn insert_task(req: &Json<AddTaskRequest>, db: &DatabaseConnection) -> AddTaskResult {
        match req.validate() {
            Ok(_) => {}
            Err(e) => {
                return AddTaskResult {
                    success: false,
                    message: Some(format!("{}", e)),
                };
            }
        }

        let prepared = entity::task::Model::prepare(&req.name, &req.create_date, &req.due_date);
        if let Ok(model) = prepared {
            match model.insert(db).await {
                Ok(_) => AddTaskResult {
                    success: true,
                    message: None,
                },
                Err(e) => AddTaskResult {
                    success: false,
                    message: Some(format!("{}", e)),
                },
            }
        } else {
            AddTaskResult {
                success: false,
                message: unsafe { Some(format!("{}", prepared.unwrap_err_unchecked())) },
            }
        }
    }

    let mut status = http::StatusCode::OK;
    let json = match session.get::<UserInfo>("user") {
        Err(e) => {
            status = http::StatusCode::INTERNAL_SERVER_ERROR;
            AddTaskResult {
                success: false,
                message: Some(format!("{}", e)),
            }
        }
        Ok(Some(user)) if user.perms.task_management => insert_task(&request, &data.sql_db).await,

        Ok(Some(user)) => {
            status = http::StatusCode::FORBIDDEN;
            AddTaskResult {
                success: false,
                message: Some(format!(
                    "User {} does not have permission to add tasks",
                    user.name
                )),
            }
        }
        Ok(_) => {
            status = http::StatusCode::UNAUTHORIZED;
            AddTaskResult {
                success: false,
                message: Some("unauthorized".to_string()),
            }
        }
    };
    simd_json::to_string(&json)
        .map_err(ErrorInternalServerError)
        .map(|x| {
            HttpResponse::Ok()
                .content_type("application/json")
                .status(status)
                .body(x)
        })
}
