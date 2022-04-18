use actix_web::{web, Scope};

pub mod hello_world;
pub mod object;
pub mod status;
pub mod task;
pub mod user;

pub fn routers() -> Scope {
    web::scope("/api")
        .route("/hello", web::get().to(hello_world::handler))
        .route("/status", web::get().to(status::handler))
        .route("/object/get", web::get().to(object::get_handler))
        .route("/object/upload", web::post().to(object::upload))
        .route(
            "/object/visibility",
            web::post().to(object::change_visibility),
        )
        .route("/object/delete", web::post().to(object::delete))
        .route("/user/add", web::post().to(user::add))
        .route("/user/login", web::post().to(user::login))
        .route("/user/logout", web::post().to(user::logout))
        .route("/user/lock", web::post().to(user::lock))
        .route("/user/unlock", web::post().to(user::unlock))
        .route("/user/delete", web::delete().to(user::delete))
        .route("/user/edit", web::post().to(user::edit))
        .route("/task/add", web::post().to(task::add_task))
        .route("/task/edit", web::post().to(task::edit_task))
        .route("/task/resolve", web::post().to(task::resolve_task))
        .route("/task/delete", web::post().to(task::delete_task))
        .route("/task/comment/add", web::post().to(task::add_comment))
        .route("/task/comment/delete", web::post().to(task::delete_comment))
        .route("/task/assign", web::post().to(task::assign))
}
