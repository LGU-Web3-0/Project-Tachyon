mod api;
mod view;

use crate::State;
use actix_files::{Directory, Files};
use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};
use actix_web::error::ErrorNotFound;
use actix_web::web::Data;
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

async fn frontend(path: web::Path<String>, data: Data<State>) -> Result<HttpResponse> {
    let path = path.into_inner();
    match data.frontend.get(path.as_str()) {
        None => Err(ErrorNotFound("not found")),
        Some(e) => Ok(HttpResponse::Ok().body(*e)),
    }
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
        .route("/frontend/{path}", web::get().to(frontend))
        .route("/", web::get().to(index))
        .service(
            Files::new("/static", static_path.as_ref())
                .show_files_listing()
                .files_listing_renderer(forbidden_index)
                .default_handler(fn_service(forbidden)),
        )
}
