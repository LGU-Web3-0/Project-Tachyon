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
        .route("/task", web::get().to(task::handler))
}
