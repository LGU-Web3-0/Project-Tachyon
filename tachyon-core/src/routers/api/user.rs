use crate::utils::IntoAnyhow;
use crate::State;
use actix_session::Session;
use actix_web::error::ErrorInternalServerError;
use actix_web::web::Json;
use actix_web::{Result, http, web, HttpResponse};
use entity::sea_orm::{ActiveModelTrait, DatabaseConnection};
use validator::Validate;

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
pub struct UserAddResult {
    success: bool,
    message: Option<String>,
}

pub async fn add(
    request: Json<UserAddRequest>,
    session: Session,
    data: web::Data<State>,
) -> Result<HttpResponse> {
    #[cfg(feature = "integration-test")]
    fn permissive(req: &Json<UserAddRequest>) -> bool {
        if let Some(true) = &req.no_session {
            true
        } else {
            false
        }
    }

    async fn is_admin(_user: &str) -> bool {
        // TODO: check admin
        true
    }

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
            entity::user::Entity::prepare(&req.name, &req.email, &req.password, &req.gpg_key);
        if let Ok(model) = prepared {
            match model.insert(db).await.anyhow() {
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
    let json = match session.get::<String>("user") {
        Err(e) => {
            status = http::StatusCode::INTERNAL_SERVER_ERROR;
            UserAddResult {
                success: false,
                message: Some(format!("{}", e)),
            }
        }
        Ok(Some(user)) if is_admin(&user).await => insert_user(&request, &data.sql_db).await,
        #[cfg(feature = "integration-test")]
        Ok(None) if permissive(&request) => insert_user(&request, &data.sql_db).await,
        Ok(_) => {
            status = http::StatusCode::UNAUTHORIZED;
            UserAddResult {
                success: false,
                message: Some("unauthorized".to_string()),
            }
        }
    };
    Ok(HttpResponse::Ok()
        .content_type("application/json;charset=utf-8")
        .status(status)
        .body(simd_json::to_string(&json).map_err(ErrorInternalServerError)?))
}

#[cfg(test)]
mod test {
    #[cfg(all(not(miri), test, feature = "integration-test"))]
    #[actix_rt::test]
    #[crate::test::serial]
    async fn it_adds_user() {
        use crate::routers::api::user::*;
        use crate::test::{startup_background, ADDRESS};
        use awc::Client;

        let _child = startup_background().await;
        let client = Client::default();
        let mut res = client
            .post(format!("http://{}/api/user/add", ADDRESS))
            .send_json(&UserAddRequest {
                name: "schrodinger".to_string(),
                email: "i@zhuyi.fan".to_string(),
                password: "123456".to_string(),
                gpg_key: entity::user::KEY_BLOCK.to_string(),
                #[cfg(feature = "integration-test")]
                no_session: Some(true),
            })
            .await
            .unwrap();
        let res: UserAddResult = res.json().await.unwrap();
        dbg!(res);
    }
}
