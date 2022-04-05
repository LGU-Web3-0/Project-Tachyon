use crate::State;
use actix_session::Session;
use actix_web::error::ErrorInternalServerError;
use actix_web::web::Json;
use actix_web::{http, web, HttpResponse, Result};
use entity::sea_orm::{ActiveModelTrait, DatabaseConnection};
use validator::Validate;
use crate::session::UserInfo;

#[derive(serde::Serialize, serde::Deserialize, Debug, Validate)]
pub struct UserAddRequest {
    #[validate(length(min = 1))]
    name: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 1))]
    password: String,
    #[validate(length(min = 1))]
    gpg_key: String,
    #[cfg(feature = "integration-test")]
    no_session: Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UserLogin {
    email: String,
    password: String,
    signature: Option<String>
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UserAddResult {
    success: bool,
    message: Option<String>,
}

pub async fn login(request: Json<UserAddRequest>,
                    session: Session,
                    data: web::Data<State>) {

}

pub async fn add(
    request: Json<UserAddRequest>,
    session: Session,
    data: web::Data<State>,
) -> Result<HttpResponse> {
    async fn insert_user(req: &Json<UserAddRequest>, db: &DatabaseConnection) -> UserAddResult {
        match req.validate() {
            Ok(_) => {}
            Err(e) => {
                return UserAddResult {
                    success: false,
                    message: Some(format!("{}", e)),
                };
            }
        }
        let prepared =
            entity::user::Model::prepare(&req.name, &req.email, &req.password, &req.gpg_key);
        if let Ok(model) = prepared {
            match model.insert(db).await {
                Ok(_) => UserAddResult {
                    success: true,
                    message: None,
                },
                Err(e) => UserAddResult {
                    success: false,
                    message: Some(format!("{}", e)),
                },
            }
        } else {
            UserAddResult {
                success: false,
                message: unsafe { Some(format!("{}", prepared.unwrap_err_unchecked())) },
            }
        }
    }
    let mut status = http::StatusCode::OK;
    let json = match session.get::<UserInfo>("user") {
        Err(e) => {
            status = http::StatusCode::INTERNAL_SERVER_ERROR;
            UserAddResult {
                success: false,
                message: Some(format!("{}", e)),
            }
        }
        Ok(Some(user)) if user.perms.user_management => insert_user(&request, &data.sql_db).await,
        #[cfg(feature = "integration-test")]
        Ok(None) if matches!(request.no_session, Some(true)) => {
            insert_user(&request, &data.sql_db).await
        }
        Ok(_) => {
            status = http::StatusCode::UNAUTHORIZED;
            UserAddResult {
                success: false,
                message: Some("unauthorized".to_string()),
            }
        }
    };
    let reply = HttpResponse::Ok()
        .content_type("application/json;charset=utf-8")
        .status(status)
        .body(simd_json::to_string(&json).map_err(ErrorInternalServerError)?);

    Ok(reply)
}

#[cfg(test)]
mod test {
    #[cfg(all(not(miri), test, feature = "integration-test"))]
    #[actix_rt::test]
    #[serial_test::serial]
    async fn it_adds_user() {
        use crate::routers::api::user::*;
        use actix_web::test;
        crate::test_env!(|app| async move {
            let req = test::TestRequest::post()
                .uri("/api/user/add")
                .set_json(&UserAddRequest {
                    name: "schrodinger".to_string(),
                    email: "i@zhuyi.fan".to_string(),
                    password: "123456".to_string(),
                    gpg_key: entity::user::KEY_BLOCK.to_string(),
                    #[cfg(feature = "integration-test")]
                    no_session: Some(true),
                })
                .to_request();
            let resp: UserAddResult = test::call_and_read_body_json(&app, req).await;
            assert!(resp.success);
        })
    }
}
