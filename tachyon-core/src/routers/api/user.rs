use crate::session::UserInfo;
use crate::{session, IntoAnyhow, State};
use actix_session::Session;
use actix_web::error::ErrorInternalServerError;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{http, web, HttpResponse, Result};
use anyhow::anyhow;
use entity::sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseBackend, DatabaseConnection,
    EntityTrait, QueryFilter, Statement,
};
use uuid::Uuid;
use validator::Validate;

pub const WRONG_PASS_ATTEMPT_THRESHOLD: i64 = 5;

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
    signature: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UserAddResult {
    success: bool,
    message: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UserLoginResult {
    success: bool,
    signature_requirement: Option<Uuid>,
    message: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UserLogoutResult {
    success: bool,
    message: Option<String>,
}

impl UserLoginResult {
    fn from_error<E: Into<anyhow::Error>>(e: E, signature_requirement: Option<Uuid>) -> Self {
        Self {
            success: false,
            signature_requirement,
            message: Some(format!("{}", e.into())),
        }
    }
    fn to_reply(&self, status: StatusCode) -> Result<HttpResponse> {
        simd_json::to_string(self)
            .map_err(ErrorInternalServerError)
            .map(|x| HttpResponse::Ok().status(status).body(x))
    }
}

pub async fn logout(session: Session) -> HttpResponse {
    match session.remove("user").ok_or("already logged out") {
        Ok(_) => HttpResponse::Ok().json(UserLogoutResult {
            success: true,
            message: None,
        }),
        Err(e) => HttpResponse::BadRequest().json(UserLogoutResult {
            success: false,
            message: Some(e.to_string()),
        }),
    }
}

pub async fn login(
    request: Json<UserLogin>,
    session: Session,
    data: web::Data<State>,
) -> Result<HttpResponse> {
    if let Ok(Some(_)) = session.get::<UserInfo>("user") {
        UserLoginResult {
            success: false,
            signature_requirement: None,
            message: Some("already logged in".to_string()),
        }
        .to_reply(StatusCode::BAD_REQUEST)
    } else {
        async fn verify_pass_and_login(
            user: &entity::user::Model,
            session: &Session,
            db: &entity::sea_orm::DatabaseConnection,
            pass: &str,
        ) -> Result<HttpResponse> {
            // TODO: handle permission
            match user.verify_password(pass) {
                Ok(true) => {
                    let query = format!(
                        r#"
                    UPDATE "user"
                    SET wrong_pass_attempt = 0
                    WHERE "id" = {}
                    "#,
                        user.id
                    );
                    let i = db
                        .execute(Statement::from_string(DatabaseBackend::Postgres, query))
                        .await
                        .anyhow();
                    match session
                        .insert(
                            "user",
                            session::UserInfo {
                                id: user.id,
                                name: user.name.clone(),
                                email: user.email.clone(),
                                perms: session::Permissions {
                                    task_management: true,
                                    file_management: true,
                                    team_management: true,
                                    user_management: true,
                                    system_management: true,
                                },
                            },
                        )
                        .anyhow()
                        .and(i)
                    {
                        Ok(_) => UserLoginResult {
                            success: true,
                            signature_requirement: None,
                            message: None,
                        }
                        .to_reply(StatusCode::OK),
                        Err(e) => UserLoginResult::from_error(e, None)
                            .to_reply(StatusCode::INTERNAL_SERVER_ERROR),
                    }
                }
                Ok(false) => {
                    let query = format!(
                        r#"
                    UPDATE "user"
                    SET wrong_pass_attempt = wrong_pass_attempt + 1
                    WHERE "id" = {}
                    "#,
                        user.id
                    );
                    match db
                        .execute(Statement::from_string(DatabaseBackend::Postgres, query))
                        .await
                    {
                        Ok(_) => UserLoginResult {
                            success: false,
                            signature_requirement: None,
                            message: Some("password mismatch".to_string()),
                        }
                        .to_reply(StatusCode::UNAUTHORIZED),
                        Err(e) => UserLoginResult::from_error(e, None)
                            .to_reply(StatusCode::INTERNAL_SERVER_ERROR),
                    }
                }
                Err(e) => {
                    UserLoginResult::from_error(e, None).to_reply(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }

        let target_user: anyhow::Result<entity::user::Model> = entity::user::Entity::find()
            .filter(entity::user::Column::Email.eq(request.email.as_str()))
            .one(&data.sql_db)
            .await
            .anyhow()
            .and_then(|x| {
                x.ok_or_else(|| anyhow!("user associated with {} not found", request.email))
            });

        if target_user.is_err() {
            return UserLoginResult::from_error(
                unsafe { target_user.unwrap_err_unchecked() },
                None,
            )
            .to_reply(StatusCode::BAD_REQUEST);
        }

        let target_user = unsafe { target_user.unwrap_unchecked() };

        if target_user.wrong_pass_attempt >= WRONG_PASS_ATTEMPT_THRESHOLD {
            match &request.signature {
                None => {
                    let uuid = Uuid::new_v4();
                    match session.insert("verification", uuid) {
                        Ok(_) => UserLoginResult {
                            success: false,
                            signature_requirement: Some(uuid),
                            message: Some("account locked".to_string()),
                        }
                        .to_reply(StatusCode::LOCKED),
                        Err(e) => UserLoginResult::from_error(e, None)
                            .to_reply(StatusCode::INTERNAL_SERVER_ERROR),
                    }
                }
                Some(sig) => {
                    match session
                        .get::<Uuid>("verification")
                        .anyhow()
                        .and_then(|x| x.ok_or_else(|| anyhow!("verification not initiated")))
                    {
                        Ok(uuid) => {
                            let message = uuid.to_string();
                            match target_user.verify_signature(&sig, &message) {
                                Ok(false) => UserLoginResult {
                                    success: false,
                                    signature_requirement: Some(uuid),
                                    message: Some("failed to verify signature".to_string()),
                                }
                                .to_reply(StatusCode::LOCKED),
                                Ok(true) => {
                                    verify_pass_and_login(
                                        &target_user,
                                        &session,
                                        &data.sql_db,
                                        &request.password,
                                    )
                                    .await
                                }
                                Err(e) => UserLoginResult::from_error(e, Some(uuid))
                                    .to_reply(StatusCode::BAD_REQUEST),
                            }
                        }
                        Err(e) => UserLoginResult::from_error(e, None)
                            .to_reply(StatusCode::INTERNAL_SERVER_ERROR),
                    }
                }
            }
        } else {
            verify_pass_and_login(&target_user, &session, &data.sql_db, &request.password).await
        }
    }
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
    simd_json::to_string(&json)
        .map_err(ErrorInternalServerError)
        .map(|x| {
            HttpResponse::Ok()
                .content_type("application/json;charset=utf-8")
                .status(status)
                .body(x)
        })
}

#[cfg(test)]
mod test {
    use actix_web::test::read_body_json;

    #[cfg(all(not(miri), test, feature = "integration-test"))]
    #[actix_rt::test]
    #[serial_test::serial]
    async fn it_adds_user() {
        use crate::routers::api::user::*;
        use actix_web::dev::ServiceResponse;
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
            for _ in 0..10 {
                let req = test::TestRequest::post()
                    .uri("/api/user/login")
                    .set_json(&UserLogin {
                        email: "i@zhuyi.fan".to_string(),
                        password: "123456".to_string(),
                        signature: None,
                    })
                    .to_request();
                let resp: ServiceResponse<_> = test::call_service(&app, req).await;
                let mut session_cookie = None;
                for i in resp.response().cookies() {
                    if i.name() == "tachyon_id" {
                        session_cookie.replace(i);
                    }
                }
                assert_eq!(resp.status(), StatusCode::OK);
                let req = test::TestRequest::post()
                    .uri("/api/user/logout")
                    .cookie(session_cookie.unwrap())
                    .to_request();
                let resp: ServiceResponse<_> = test::call_service(&app, req).await;
                assert_eq!(resp.status(), StatusCode::OK);
            }
        })
    }

    #[cfg(all(not(miri), test, feature = "integration-test"))]
    #[actix_rt::test]
    #[serial_test::serial]
    async fn it_locks_and_unlocks_user() {
        use crate::routers::api::user::*;
        use actix_web::dev::ServiceResponse;
        use actix_web::test;
        let helper = crate::test::GPGHelper::new();
        crate::test_env!(|app| async move {
            let req = test::TestRequest::post()
                .uri("/api/user/add")
                .set_json(&UserAddRequest {
                    name: "schrodinger".to_string(),
                    email: "i@zhuyi.fan".to_string(),
                    password: "123456".to_string(),
                    gpg_key: helper.armored_public_key(),
                    #[cfg(feature = "integration-test")]
                    no_session: Some(true),
                })
                .to_request();
            let resp: UserAddResult = test::call_and_read_body_json(&app, req).await;
            assert!(resp.success);
            for _ in 0..10 {
                let req = test::TestRequest::post()
                    .uri("/api/user/login")
                    .set_json(&UserLogin {
                        email: "i@zhuyi.fan".to_string(),
                        password: "1234567".to_string(),
                        signature: None,
                    })
                    .to_request();
                test::call_service(&app, req).await;
            }

            let req = test::TestRequest::post()
                .uri("/api/user/login")
                .set_json(&UserLogin {
                    email: "i@zhuyi.fan".to_string(),
                    password: "1234567".to_string(),
                    signature: None,
                })
                .to_request();
            let resp: ServiceResponse<_> = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::LOCKED);

            let mut session_cookie = None;
            for i in resp.response().cookies() {
                if i.name() == "tachyon_id" {
                    session_cookie.replace(i.into_owned());
                }
            }
            let json: UserLoginResult = read_body_json(resp).await;
            let message = json.signature_requirement.unwrap().to_string();
            let signature = helper.signature(message);
            println!("{}", signature);
            let req = test::TestRequest::post()
                .uri("/api/user/login")
                .cookie(session_cookie.unwrap())
                .set_json(&UserLogin {
                    email: "i@zhuyi.fan".to_string(),
                    password: "123456".to_string(),
                    signature: Some(signature),
                })
                .to_request();
            let res: UserLoginResult = test::call_and_read_body_json(&app, req).await;
            dbg!(&res);
            assert!(res.success);
        })
    }
}
