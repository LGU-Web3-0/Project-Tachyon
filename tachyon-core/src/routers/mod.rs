//! Router is core function in the api. It basically works likes this:
//!
//! ```bash
//!               Request
//! HttpClient --------------> HttpServer
//!                                |      \
//!                                |       \ route to
//!                                |        \
//!                                |         \            ORM
//!                                |        Api Handler: ------> PostgresQL
//!                                |           |                     |
//!                                            |          Result     | CRUD
//!                              Return the updated data <------     |
//!                                           /
//!                                |         /
//!                                |        /
//!                                |       /
//!                      *if api them return json
//!                      *if view hand to frontend component to deal and bundle
//! HttpClient <----------------  and return html css js ...
//!
//! ```
//! The router here is responsible for the route_to function.
//! Key Implementation:
//! * The router routes based on the url, which is called a path in nowadays web framework.
//! As you can see in the parameter of our routers function:
//! ```rust
//! pub fn routers<S: AsRef<Path>>(static_path: S) -> Scope {
//!     web::scope("")               
//!         .service(api::routers())
//!         .service(view::routers())
//!         .route("/frontend/{path}", web::get().to(frontend))
//!         .route("/", web::get().to(index))
//!         .service(
//!             Files::new("/static", static_path.as_ref())
//!                 .show_files_listing()
//!                 .files_listing_renderer(forbidden_index)
//!                 .default_handler(fn_service(forbidden)),
//!         )
//! }
//!
//! ```
//! the router then parse the path to get parameter from it.
//!

mod api;
mod view;

use crate::session::UserInfo;
use actix_files::{Directory, Files};
use actix_session::Session;
use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};
use actix_web::error::ErrorNotFound;
use actix_web::{web, HttpRequest, HttpResponse, Result, Scope};
use std::path::Path;
use tachyon_template::AsyncRenderOnce;
pub use view::error::error_handler;

async fn forbidden(req: ServiceRequest) -> Result<ServiceResponse> {
    Ok(ServiceResponse::new(
        req.into_parts().0,
        HttpResponse::Forbidden().body(()),
    ))
}

fn forbidden_index(_: &Directory, req: &HttpRequest) -> std::io::Result<ServiceResponse> {
    Ok(ServiceResponse::new(
        req.clone(),
        HttpResponse::Forbidden().body(()),
    ))
}

async fn frontend(path: web::Path<String>) -> Result<HttpResponse> {
    let path = path.into_inner();
    match tachyon_frontend::TARGETS.get(path.as_str()) {
        None => Err(ErrorNotFound("not found")),
        Some(e) => Ok(HttpResponse::Ok().body(*e)),
    }
}

async fn index(session: Session) -> Result<HttpResponse> {
    if let Ok(Some(_)) = session.get::<UserInfo>("user") {
        HttpResponse::TemporaryRedirect()
            .append_header(("Location", "/view/dashboard"))
            .await
    } else {
        tachyon_template::IndexTemplate::new("Project Tachyon")
            .render_response()
            .await
    }
}

pub fn routers<S: AsRef<Path>>(static_path: S) -> Scope {
    web::scope("")
        .service(api::routers())
        .service(view::routers())
        .route("/frontend/{path}", web::get().to(frontend))
        .route("/", web::get().to(index))
        .service(
            Files::new("/static", static_path.as_ref())
                .show_files_listing()
                .files_listing_renderer(forbidden_index)
                .default_handler(fn_service(forbidden)),
        )
}

#[cfg(test)]
mod test {
    use actix_web::dev::ServiceResponse;
    use actix_web::test;

    #[cfg(all(not(miri), test, feature = "integration-test"))]
    #[actix_rt::test]
    #[serial_test::serial]
    async fn it_gets_front_resources() {
        crate::test_env!(|app| async move {
            for i in [
                "/frontend/tachyon.js",
                "/frontend/tachyon.js.map",
                "/frontend/main.css",
                "/frontend/main.css.map",
            ] {
                let req = test::TestRequest::get().uri(i).to_request();
                let resp: ServiceResponse<_> = test::call_service(&app, req).await;
                log::debug!("status: {:?}", resp.status());
                assert!(resp.status().is_success())
            }

            for i in ["/frontend/aaa", "/frontend/bbb"] {
                let req = test::TestRequest::get().uri(i).to_request();
                let resp: ServiceResponse<_> = test::call_service(&app, req).await;
                assert!(!resp.status().is_success())
            }
        });
    }

    #[cfg(all(not(miri), test, feature = "integration-test"))]
    #[actix_rt::test]
    #[serial_test::serial]
    async fn it_gets_index_page() {
        crate::test_env!(|app| async move {
            let req = test::TestRequest::get().uri("/").to_request();
            let resp: ServiceResponse<_> = test::call_service(&app, req).await;
            log::debug!("status: {:?}", resp.status());
            assert!(resp.status().is_success())
        });
    }
}
