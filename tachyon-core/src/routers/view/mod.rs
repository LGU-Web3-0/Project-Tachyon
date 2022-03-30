use actix_web::{web, Scope};
mod hello;
pub fn routers() -> Scope {
    web::scope("/view")
        .route("/hello", web::get().to(hello::handler))
}