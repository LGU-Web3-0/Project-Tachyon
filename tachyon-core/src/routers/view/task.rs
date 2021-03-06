use crate::session::UserInfo;
use crate::State;
use actix_session::Session;
use actix_web::error::{ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized};
use actix_web::web::Data;
use actix_web::web::Path;
use actix_web::{HttpResponse, Result};
use entity::sea_orm::{DbBackend, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Statement};
use sea_query::{Expr, Order};
use tachyon_template::view::{Comment, TaskDetailTemplate, UserData};
use tachyon_template::{view::TaskTemplate, AsyncRenderOnce};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TaskRequest {
    search_string: Option<String>,
    page_no: Option<usize>,
    page_size: Option<usize>,
}

fn convert_task_info<I>(task: I, email: &str) -> Vec<tachyon_template::view::TaskItem>
where
    I: Iterator<Item = entity::task::Model>,
{
    task.map(|mut t| {
        t.description.truncate(64);
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
    state: Data<State>,
) -> Result<HttpResponse> {
    let user = session
        .get::<UserInfo>("user")
        .map_err(ErrorInternalServerError)
        .and_then(|data| data.ok_or_else(|| ErrorUnauthorized("no login info")))?;
    let info = info.into_inner();
    let task = entity::task::Entity::find_by_id(info.id)
        .one(&state.sql_db)
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("no such task"))?;
    let assigned_users = entity::user::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"SELECT * 
                FROM "user" 
                JOIN task_user_assignment ON task_user_assignment.task_id = $1 AND task_user_assignment.user_id = "user".id
                ORDER BY "user".id"#,
            vec![info.id.into()],
        ))
        .all(&state.sql_db)
        .await
        .map_err(ErrorInternalServerError)?
        .into_iter()
        .map(|u| UserData::new(u.name, u.email))
        .collect();
    let comment = entity::task_discussion::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"SELECT * 
                FROM task_discussion 
                WHERE task_discussion.task_id = $1
                ORDER BY task_discussion.id DESC"#,
            vec![info.id.into()],
        ))
        .all(&state.sql_db)
        .await
        .map_err(ErrorInternalServerError)?;

    let mut comment_and_user = Vec::new();
    for i in comment.into_iter() {
        if let Some(user) = entity::user::Entity::find_by_id(i.user_id)
            .one(&state.sql_db)
            .await
            .map_err(ErrorInternalServerError)?
        {
            let real_comment = Comment::new(
                i.id,
                i.content,
                i.update_time,
                UserData::new(user.name, user.email),
            );
            comment_and_user.push(real_comment);
        }
    }
    let template = TaskDetailTemplate::new(
        user.perms.user_management,
        "Task Detail | Project Tachyon",
        user.email.clone(),
        info.id,
        task.name,
        task.create_date,
        task.finish_date,
        task.due_date,
        assigned_users,
        comment_and_user,
        task.description,
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
        Some(user) => {
            let mut page = entity::task::Entity::find();
            if let Some(keywords) = &request.search_string {
                log::debug!("search string: {}", keywords);
                let expr = Expr::cust_with_values(
                    "task_search_vector @@ plainto_tsquery(?)",
                    vec![keywords.to_string()],
                );
                page = page.filter(expr);
            }
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
                user.perms.user_management,
                "MyTasks | Project Tachyon",
                user.email,
                converted,
                page_size,
                prev_page,
                next_page,
                request.search_string.as_ref(),
            )
            .render_response()
            .await
        }
    }
}
