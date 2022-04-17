use crate::session::UserInfo;
use crate::State;
use actix_session::Session;
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::web::Data;
use actix_web::web::Path;
use actix_web::{HttpResponse, Result};
use entity::sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};
use sea_query::Order;
use tachyon_template::view::{Comment, TaskDetailTemplate, UserData};
use tachyon_template::{view::TaskTemplate, AsyncRenderOnce};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TaskRequest {
    page_no: Option<usize>,
    page_size: Option<usize>,
}

fn convert_task_info<I>(task: I, email: &str) -> Vec<tachyon_template::view::TaskItem>
where
    I: Iterator<Item = entity::task::Model>,
{
    task.map(|t| {
        tachyon_template::view::TaskItem::new(t.id, email.to_owned(), t.name, t.description)
    })
    .collect()
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TaskDetailRequest {
    id: i64,
}

pub async fn detail(
    info: Path<TaskDetailRequest>,
    session: Session,
    _state: Data<State>,
) -> Result<HttpResponse> {
    let user = session
        .get::<UserInfo>("user")
        .map_err(ErrorInternalServerError)
        .and_then(|data| data.ok_or_else(|| ErrorUnauthorized("no login info")))?;
    let info = info.into_inner();
    let template = TaskDetailTemplate::new(
        "Task | Project Tachyon",
            user.email,
            info.id,
            "Test",
            chrono::Utc::now(),
            Some(chrono::Utc::now()),
            vec![
                UserData::new("123", "a@b.com"),
                UserData::new("Schrodinger ZHU Yifan", "i@zhuyi.fan")
            ],
            vec![
                Comment::new(0, "hello, world", chrono::Utc::now(),UserData::new("Schrodinger ZHU Yifan", "i@zhuyi.fan")),
                Comment::new(1, "- A \n - B \n - C ```123```", chrono::Utc::now(),UserData::new("Schrodinger ZHU Yifan", "i@zhuyi.fan")),
                Comment::new(3, "- A \n - B \n - C ```123```", chrono::Utc::now(),UserData::new("123", "a@b.com")),
            ],
        "
The Chinese University of Hong Kong, Shenzhen （CUHK-Shenzhen）was founded in accordance with the Regulations of the People’s Republic of China on Chinese-foreign Cooperation in Running Schools upon approval by the Ministry of Education. The University is committed to providing top-quality higher education that features an integration of the East and the West and fostering an enriching research environment. It is CUHK-Shenzhen’s mission to cultivate innovative talents with a global perspective, Chinese cultural traditions and social responsibilities.
",
    );
    template.render_response().await
}

pub async fn handler(
    request: actix_web::web::Query<TaskRequest>,
    data: Data<State>,
    session: Session,
) -> Result<HttpResponse> {
    match session.get::<UserInfo>("user")? {
        None => Err(ErrorUnauthorized("login info not found")),
        Some(_user) => {
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
