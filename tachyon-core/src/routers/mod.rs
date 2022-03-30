mod api;
mod view;

use actix_web::{Scope, web};

pub fn routers() -> Scope {
    web::scope("")
        .service(api::routers())
        .service(view::routers())
}