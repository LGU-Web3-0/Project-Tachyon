use crate::routers::api::user::WRONG_PASS_ATTEMPT_THRESHOLD;
use crate::session::UserInfo;
use crate::State;
use actix_session::Session;
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::web::Data;
use actix_web::{HttpResponse, Result};
use entity::sea_orm::{EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use sea_query::{Expr, Order};
use tachyon_template::{view::TaskTemplate, view::UserTemplate, AsyncRenderOnce};


pub async fn handler(
    request: actix_web::web::Query<UserRequest>,
    data: Data<State>,
    session: Session,
) -> Result<HttpResponse> {
    match session.get::<UserInfo>("user")? {
        None => Err(ErrorUnauthorized("login info not found")),
        Some(user) => {
             UserTemplate::new(
                 "MyTasks | Project Tachyon",

                 )
                 .render_response()
                 .await
        }
    }



}
