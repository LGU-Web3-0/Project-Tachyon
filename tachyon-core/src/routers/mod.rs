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
pub use view::error::add_error_header;

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
