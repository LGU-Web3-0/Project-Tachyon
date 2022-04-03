use actix_web::{web, Scope};

pub mod hello_world;
pub mod object;
pub mod status;
pub mod user;

pub fn routers() -> Scope {
    web::scope("/api")
        .route("/hello", web::get().to(hello_world::handler))
        .route("/status", web::get().to(status::handler))
        .route("/object/get", web::get().to(object::get_handler))
        .route("/user/add", web::post().to(user::add))
}
