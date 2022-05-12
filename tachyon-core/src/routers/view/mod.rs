use actix_web::{web, Scope};

pub mod dashboard;
pub mod error;
pub mod hello;
pub mod object;
pub mod task;
pub mod user;

pub fn routers() -> Scope {
    web::scope("/view")
        .route("/hello", web::get().to(hello::handler))
        .route("/dashboard", web::get().to(dashboard::handler))
        .route("/user", web::get().to(user::handler))
        .route("/object", web::get().to(object::handler))
        .route("/task/{id}/detail", web::get().to(task::detail))
        .route("/task", web::get().to(task::handler))
}

#[cfg(test)]
mod test {
    #[cfg(all(not(miri), test, feature = "integration-test"))]
    #[actix_rt::test]
    #[serial_test::serial]
    async fn it_renders_view() {
        use crate::StatusCode;
        use actix_web::cookie::Cookie;
        use actix_web::dev::ServiceResponse;
        use actix_web::test;

        crate::test_env!(|app| async move {
            crate::with_login_cookie!(app, |app, cookie: Cookie<'static>| async move {
                let req = test::TestRequest::get().uri("/view/hello").to_request();
                let resp: ServiceResponse<_> = test::call_service(&app, req).await;
                assert_eq!(resp.status(), StatusCode::OK);
                for i in [
                    "/view/dashboard",
                    "/view/user",
                    "/view/user?page_size=1&page_no=0&search_string=schrodinger",
                    "/view/user?page_size=1&page_no=0",
                    "/view/object",
                    "/view/object?page=1",
                    "/view/task",
                    "/view/task?page_size=1&page_no=0&search_string=aaa",
                    "/view/task?page_size=1&page_no=0",
                ] {
                    let req = test::TestRequest::get().uri(i).to_request();
                    let resp: ServiceResponse<_> = test::call_service(&app, req).await;
                    assert_ne!(resp.status(), StatusCode::OK);
                    let req = test::TestRequest::get()
                        .uri(i)
                        .cookie(cookie.clone())
                        .to_request();
                    let resp: ServiceResponse<_> = test::call_service(&app, req).await;
                    assert_eq!(resp.status(), StatusCode::OK);
                }
            });
        })
    }
}
