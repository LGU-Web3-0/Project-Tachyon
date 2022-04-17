use crate::session::UserInfo;
use actix_session::Session;
use actix_web::error::ErrorUnauthorized;
use actix_web::HttpResponse;
use actix_web::Result;
use tachyon_template::view::ObjectTemplate;
use tachyon_template::AsyncRenderOnce;

pub async fn handler(session: Session) -> Result<HttpResponse> {
    match session.get::<UserInfo>("user")? {
        None => Err(ErrorUnauthorized("login info not found")),
        Some(user) => {
            ObjectTemplate::new("Dashboard | Project Tachyon", user.email)
                .render_response()
                .await
        }
    }
}
