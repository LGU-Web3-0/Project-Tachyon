use actix_web::{web, Scope};

mod hello_world;
mod status;
mod object;
pub fn routers() -> Scope {
    web::scope("/api")
        .route("/hello", web::get().to(hello_world::handler))
        .route("/status", web::get().to(status::handler))
        .route("/object/get", web::get().to(object::get_handler))
}
