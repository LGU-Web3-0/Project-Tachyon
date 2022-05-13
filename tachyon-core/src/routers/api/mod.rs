//! Our router is RESTful. Which means:
//! REST stands for Representational State Transfer. To follow it in our routes, we use a convention called RESTful Routing. RESTful routing is a set of standards used in many different languages to create efficient, reusable routes. It aims to map HTTP methods (GET, POST, PATCH, DELETE) and CRUD actions (`Create`, `Read`, `Update`, `Destroy`) together in a conventional pattern. This makes it easier for other developers to understand and navigate an application and results in clean, consistent URL paths for users.
//!Essentially, RESTful routing is a naming pattern. It asserts that routes completing certain common actions (like creating, updating, or deleting objects) have names and paths that accurately reflect what they're doing, with which CRUD and HTTP verbs, on what type of object.

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
