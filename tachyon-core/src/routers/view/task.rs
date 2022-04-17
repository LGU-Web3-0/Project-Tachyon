use crate::session::UserInfo;
use crate::State;
use actix_session::Session;
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::web::Data;
use actix_web::{HttpResponse, Result};
use entity::sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};
use sea_query::Order;
use tachyon_template::{view::TaskTemplate, AsyncRenderOnce};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TaskRequest {
    page_no: Option<usize>,
    page_size: Option<usize>,
}

fn convert_task_info<I>(task: I, email: &String) -> Vec<tachyon_template::view::TaskItem>
where
    I: Iterator<Item = entity::task::Model>,
{
    task.map(|t| tachyon_template::view::TaskItem::new(
            t.id, 
            email.clone(),
            t.name))
        .collect()
}

pub async fn handler(
    request: actix_web::web::Query<TaskRequest>,
    data: Data<State>,
    session: Session,
) -> Result<HttpResponse> {
    match session.get::<UserInfo>("user")? {
        None => Err(ErrorUnauthorized("login info not found")),
        Some(user) => {
            let page = entity::task::Entity::find();
            let page_size = request.page_size.unwrap_or(10);
            let paginator = page
                .order_by(entity::task::Column::Id, Order::Asc)
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
            let converted = convert_task_info(items.into_iter(), &user.email);
            TaskTemplate::new(
                "MyTasks | Project Tachyon",
                user.email,
                converted,
                page_size,
                prev_page,
                next_page,
            )
            .render_response()
            .await
        }
    }
}
