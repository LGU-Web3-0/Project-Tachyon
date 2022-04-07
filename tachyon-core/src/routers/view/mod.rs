use actix_web::{web, Scope};
pub mod error;
pub mod hello;
pub fn routers() -> Scope {
    web::scope("/view").route("/hello", web::get().to(hello::handler))
}
