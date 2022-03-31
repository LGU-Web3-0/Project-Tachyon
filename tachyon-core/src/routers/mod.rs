mod api;
mod view;

use actix_files::{Directory, Files};
use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};
use actix_web::{web, HttpRequest, HttpResponse, Result, Scope};
use std::path::Path;
use tachyon_template::AsyncRenderOnce;

async fn forbidden(req: ServiceRequest) -> Result<ServiceResponse> {
    Ok(ServiceResponse::new(
        req.into_parts().0,
        HttpResponse::Forbidden().body("forbidden"),
    ))
}

fn forbidden_index(_: &Directory, req: &HttpRequest) -> std::io::Result<ServiceResponse> {
    Ok(ServiceResponse::new(
        req.clone(),
        HttpResponse::Forbidden().body("forbidden"),
    ))
}

async fn tachyonjs() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/x-ecmascript;charset=utf-8")
        .body(tachyon_typescript::TACHYON_SCRIPT)
}

async fn index() -> Result<HttpResponse> {
    tachyon_template::IndexTemplate::new("Project Tachyon")
        .render_response()
        .await
}

pub fn routers<S: AsRef<Path>>(static_path: S) -> Scope {
    web::scope("")
        .service(api::routers())
        .service(view::routers())
        .route("/static/tachyon.js", web::get().to(tachyonjs))
        .route("/", web::get().to(index))
        .service(
            Files::new("/static", static_path.as_ref())
                .show_files_listing()
                .files_listing_renderer(forbidden_index)
                .default_handler(fn_service(forbidden)),
        )
}
