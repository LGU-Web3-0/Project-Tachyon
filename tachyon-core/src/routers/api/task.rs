use crate::session::UserInfo;
use crate::State;
use actix_session::Session;
use actix_web::error::{
    ErrorBadRequest, ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized,
};
use actix_web::web::Json;
use actix_web::{http, web, HttpResponse, Result};
use entity::sea_orm::entity::prelude::*;
use entity::sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};
use validator::Validate;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AddTaskResult {
    success: bool,
    message: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AssignTaskResult {
    success: bool,
    message: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct EditTaskResult {
    success: bool,
    message: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ResolveTaskResult {
    success: bool,
    message: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DeleteTaskResult {
    success: bool,
    message: Option<String>,
}

#[derive(PartialEq, serde::Serialize, serde::Deserialize, Debug, Validate)]
pub struct AddTaskRequest {
    #[validate(length(min = 1))]
    name: String,
    create_date: DateTimeUtc,
    due_date: DateTimeUtc,
    description: String,
}

#[derive(PartialEq, serde::Serialize, serde::Deserialize, Debug, Validate)]
pub struct AssignTaskRequest {
    task_id: i64,
    user_id: i64,
    assign_date: DateTimeUtc,
}

#[derive(PartialEq, serde::Serialize, serde::Deserialize, Debug, Validate)]
pub struct EditTaskRequest {
    id: i64,
    updated_description: String,
}

#[derive(PartialEq, serde::Serialize, serde::Deserialize, Debug, Validate)]
pub struct ResolveTaskRequest {
    id: i64,
    finish_date: DateTimeUtc,
}

#[derive(PartialEq, serde::Serialize, serde::Deserialize, Debug, Validate)]
pub struct DeleteTaskRequest {
    id: i64,
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

        let prepared = entity::task::Model::prepare(
            &req.name,
            &req.create_date,
            &req.due_date,
            &req.description,
        );
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

pub async fn edit_task(
    request: Json<EditTaskRequest>,
    session: Session,
    data: web::Data<State>,
) -> Result<HttpResponse> {
    let mut status = http::StatusCode::OK;
    let json = match session.get::<UserInfo>("user") {
        Err(e) => {
            status = http::StatusCode::INTERNAL_SERVER_ERROR;
            EditTaskResult {
                success: false,
                message: Some(format!("{}", e)),
            }
        }
        Ok(Some(user)) if user.perms.task_management => {
            let task = entity::task::Entity::find_by_id(request.id)
                .one(&data.sql_db)
                .await
                .map_err(ErrorBadRequest)?
                .ok_or_else(|| ErrorNotFound("no such task"))?;

            let mut active_task: entity::task::ActiveModel = task.into();
            let upd_des: String = request.updated_description.clone();
            active_task.description = ActiveValue::Set(upd_des);
            match active_task
                .update(&data.sql_db)
                .await
                .map_err(ErrorInternalServerError)
            {
                Ok(_) => EditTaskResult {
                    success: true,
                    message: None,
                },
                Err(e) => EditTaskResult {
                    success: false,
                    message: Some(format!("{}", e)),
                },
            };

            EditTaskResult {
                success: true,
                message: None,
            }
        }

        Ok(_) => {
            status = http::StatusCode::UNAUTHORIZED;
            EditTaskResult {
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

pub async fn resolve_task(
    request: Json<ResolveTaskRequest>,
    session: Session,
    data: web::Data<State>,
) -> Result<HttpResponse> {
    let mut status = http::StatusCode::OK;
    let json = match session.get::<UserInfo>("user") {
        Err(e) => {
            status = http::StatusCode::INTERNAL_SERVER_ERROR;
            EditTaskResult {
                success: false,
                message: Some(format!("{}", e)),
            }
        }
        Ok(Some(user)) if user.perms.task_management => {
            let task = entity::task::Entity::find_by_id(request.id)
                .one(&data.sql_db)
                .await
                .map_err(ErrorBadRequest)?
                .ok_or_else(|| ErrorNotFound("no such task"))?;

            let mut active_task: entity::task::ActiveModel = task.into();
            let finish: DateTimeUtc = request.finish_date;
            active_task.finish_date = ActiveValue::Set(Some(finish));
            match active_task
                .update(&data.sql_db)
                .await
                .map_err(ErrorInternalServerError)
            {
                Ok(_) => EditTaskResult {
                    success: true,
                    message: None,
                },
                Err(e) => EditTaskResult {
                    success: false,
                    message: Some(format!("{}", e)),
                },
            };

            EditTaskResult {
                success: true,
                message: None,
            }
        }

        Ok(_) => {
            status = http::StatusCode::UNAUTHORIZED;
            EditTaskResult {
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

pub async fn delete_task(
    request: Json<DeleteTaskRequest>,
    session: Session,
    data: web::Data<State>,
) -> Result<HttpResponse> {
    match session.get::<UserInfo>("user").unwrap_or(None) {
        None => Ok(HttpResponse::Unauthorized().finish()),
        Some(_user_info) => {
            let task = entity::task::Entity::find_by_id(request.id)
                .one(&data.sql_db)
                .await
                .map_err(ErrorBadRequest)?
                .ok_or_else(|| ErrorNotFound("no such task"))?;
            task.delete(&data.sql_db)
                .await
                .map_err(ErrorInternalServerError)?;
            let json = DeleteTaskResult {
                success: true,
                message: Some("delete task successfully!".to_string()),
            };

            let status = http::StatusCode::OK;
            simd_json::to_string(&json)
                .map_err(ErrorInternalServerError)
                .map(|x| {
                    HttpResponse::Ok()
                        .content_type("application/json")
                        .status(status)
                        .body(x)
                })
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AddCommentRequest {
    pub task_id: i64,
    pub content: String,
}

pub async fn add_comment(
    request: Json<AddCommentRequest>,
    session: Session,
    data: web::Data<State>,
) -> Result<HttpResponse> {
    match session.get::<UserInfo>("user").unwrap_or(None) {
        None => Err(ErrorUnauthorized("no login info")),
        Some(user_info) => {
            let request = request.into_inner();
            let comment = entity::task_discussion::ActiveModel {
                id: ActiveValue::NotSet,
                task_id: ActiveValue::Set(request.task_id),
                update_time: ActiveValue::Set(chrono::Utc::now()),
                user_id: ActiveValue::Set(user_info.id),
                content: ActiveValue::Set(request.content),
            };
            comment
                .insert(&data.sql_db)
                .await
                .map_err(ErrorInternalServerError)?;
            Ok(HttpResponse::Ok().finish())
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteCommentRequest {
    pub comment_id: i64,
}

pub async fn delete_comment(
    request: Json<DeleteCommentRequest>,
    session: Session,
    data: web::Data<State>,
) -> Result<HttpResponse> {
    match session.get::<UserInfo>("user").unwrap_or(None) {
        None => Err(ErrorUnauthorized("no login info")),
        Some(user_info) => {
            let request = request.into_inner();
            let comment = entity::task_discussion::Entity::find_by_id(request.comment_id)
                .one(&data.sql_db)
                .await
                .map_err(ErrorBadRequest)?
                .ok_or_else(|| ErrorNotFound("no such comment"))?;
            if comment.user_id != user_info.id {
                return Err(ErrorUnauthorized("no permission"));
            }
            comment
                .delete(&data.sql_db)
                .await
                .map_err(ErrorInternalServerError)?;
            Ok(HttpResponse::Ok().finish())
        }
    }
}

pub async fn assign(
    request: Json<AssignTaskRequest>,
    session: Session,
    data: web::Data<State>,
) -> Result<HttpResponse> {
    async fn assign_task(
        req: &Json<AssignTaskRequest>,
        db: &DatabaseConnection,
    ) -> AssignTaskResult {
        match req.validate() {
            Ok(_) => {}
            Err(e) => {
                return AssignTaskResult {
                    success: false,
                    message: Some(format!("{}", e)),
                };
            }
        }

        let prepared = entity::task_user_assignment::Model::prepare(
            req.task_id.clone(),
            req.user_id.clone(),
            req.assign_date.clone(),
        );
        if let Ok(model) = prepared {
            match model.insert(db).await {
                Ok(_) => AssignTaskResult {
                    success: true,
                    message: None,
                },
                Err(e) => AssignTaskResult {
                    success: false,
                    message: Some(format!("{}", e)),
                },
            }
        } else {
            AssignTaskResult {
                success: false,
                message: unsafe { Some(format!("{}", prepared.unwrap_err_unchecked())) },
            }
        }
    }
    let mut status = http::StatusCode::OK;
    let json = match session.get::<UserInfo>("user") {
        Err(e) => {
            status = http::StatusCode::INTERNAL_SERVER_ERROR;
            AssignTaskResult {
                success: false,
                message: Some(format!("{}", e)),
            }
        }
        Ok(Some(user)) if user.perms.team_management => assign_task(&request, &data.sql_db).await,

        Ok(Some(user)) => {
            status = http::StatusCode::FORBIDDEN;
            AssignTaskResult {
                success: false,
                message: Some(format!(
                    "User {} does not have permission to add tasks",
                    user.name
                )),
            }
        }
        Ok(_) => {
            status = http::StatusCode::UNAUTHORIZED;
            AssignTaskResult {
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
