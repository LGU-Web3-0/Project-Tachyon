mod api;
mod view;

use actix_web::{web, Scope};

pub fn routers() -> Scope {
    web::scope("")
        .service(api::routers())
        .service(view::routers())
}
