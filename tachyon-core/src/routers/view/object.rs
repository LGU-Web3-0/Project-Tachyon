use crate::session::UserInfo;
use actix_session::Session;
use actix_web::error::ErrorUnauthorized;
use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::web::{Data, Query};
use tachyon_template::view::{ObjectItem, ObjectTemplate};
use tachyon_template::AsyncRenderOnce;
use crate::State;
use entity::sea_orm::EntityTrait;
use entity::sea_orm::QueryOrder;
use entity::sea_orm::PaginatorTrait;


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ObjectRequest {
    page: Option<usize>,
}

pub async fn handler(session: Session, data: Data<State>, request: Query<ObjectRequest>) -> Result<HttpResponse> {
    match session.get::<UserInfo>("user")? {
        None => Err(ErrorUnauthorized("login info not found")),
        Some(user) => {
            let paginator = entity::object::Entity::find()
                .order_by_asc(entity::object::Column::Uuid)
                .paginate(&data.sql_db, 5);
            let current_page = request.page.unwrap_or(0);
            let current = paginator.fetch_page(current_page).await.unwrap_or_default()
                .into_iter()
                .map(|object| {
                    ObjectItem {
                        uuid: object.uuid,
                        name: object.name,
                        mimetype: object.mimetype,
                        uploaded_at: object.upload_time,
                        visibility: object.visibility,
                    }
                })
                .collect::<Vec<ObjectItem>>();
            let next = paginator.num_pages().await
                .map(|num_pages| if current_page + 1 < num_pages { Some(current_page + 1) } else { None })
                .unwrap_or(None);
            let prev = if current_page != 0 { Some(current_page - 1) } else { None };
            ObjectTemplate::new("Object | Project Tachyon", user.email, current, current_page, next, prev)
                .render_response()
                .await
        }
    }
}
