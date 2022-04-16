use actix_web::{web, Scope};
pub mod dashboard;
pub mod error;
pub mod hello;
pub mod user;
pub fn routers() -> Scope {
    web::scope("/view")
        .route("/hello", web::get().to(hello::handler))
        .route("/dashboard", web::get().to(dashboard::handler))
        .route("/user", web::get().to(user::handler))
}
