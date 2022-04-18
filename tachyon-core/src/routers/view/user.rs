use crate::routers::api::user::WRONG_PASS_ATTEMPT_THRESHOLD;
use crate::session::UserInfo;
use crate::State;
use actix_session::Session;
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::web::Data;
use actix_web::{HttpResponse, Result};
use entity::sea_orm::{EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use sea_query::{Expr, Order};
use tachyon_template::{view::UserTemplate, AsyncRenderOnce};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserRequest {
    page_no: Option<usize>,
    page_size: Option<usize>,
    search_string: Option<String>,
}

fn convert_user_info<I>(user: I) -> Vec<tachyon_template::view::UserItem>
where
    I: Iterator<Item = entity::user::Model>,
{
    user.filter_map(|user| {
        user.fingerprint()
            .map(|f| {
                tachyon_template::view::UserItem::new(
                    user.id,
                    user.email,
                    user.name,
                    user.wrong_pass_attempt >= WRONG_PASS_ATTEMPT_THRESHOLD,
                    f,
                )
            })
            .ok()
    })
    .collect()
}

pub async fn handler(
    request: actix_web::web::Query<UserRequest>,
    data: Data<State>,
    session: Session,
) -> Result<HttpResponse> {
    match session.get::<UserInfo>("user")? {
        None => Err(ErrorUnauthorized("login info not found")),
        Some(user) => {
            let mut page = entity::user::Entity::find();

            if let Some(keywords) = &request.search_string {
                log::debug!("search string: {}", keywords);
                let expr = Expr::cust_with_values(
                    "user_search_vector @@ plainto_tsquery(?)",
                    vec![keywords.to_string()],
                );
                page = page.filter(expr);
            }
            let page_size = request.page_size.unwrap_or(10);
            let paginator = page
                .order_by(entity::user::Column::Id, Order::Asc)
                .paginate(&data.sql_db, page_size);
            let items = paginator
                .fetch_page(request.page_no.unwrap_or(0))
                .await
                .map_err(ErrorInternalServerError)?;
            let num_pages = paginator
                .num_pages()
                .await
                .map_err(ErrorInternalServerError)?;
            let prev_page = match request.page_no {
                None | Some(0) => None,
                Some(no) => Some(no - 1),
            };
            let next_page = match request.page_no.unwrap_or(0) {
                n if n + 1 >= num_pages => None,
                n => Some(n + 1),
            };
            log::debug!("select {} items", items.len());
            let converted = convert_user_info(items.into_iter());
            UserTemplate::new(
                user.perms.user_management,
                "User | Project Tachyon",
                user.email,
                converted,
                page_size,
                prev_page,
                next_page,
                request.search_string.clone(),
            )
            .render_response()
            .await
        }
    }
}
