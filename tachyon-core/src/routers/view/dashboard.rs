use crate::session::UserInfo;
use crate::State;
use actix_session::Session;
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::web::Data;
use actix_web::{HttpResponse, Result};
use entity::sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
use entity::sea_orm::{DatabaseBackend, Statement};
use tachyon_template::view::RelatedTask;
use tachyon_template::{view::DashboardTemplate, AsyncRenderOnce};

pub async fn get_related_tasks(
    user_info: &UserInfo,
    data: &Data<State>,
) -> Result<Vec<RelatedTask>> {
    let tasks: Vec<entity::task::Model> = entity::task::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"
            SELECT * FROM task
            JOIN task_user_assignment
            ON task_user_assignment.user_id = $1
            ORDER BY task.create_date DESC
            "#,
            vec![user_info.id.into()],
        ))
        .all(&data.sql_db)
        .await
        .map_err(ErrorInternalServerError)?;

    let mut related_tasks = Vec::new();
    for i in tasks.into_iter() {
        let count = entity::task_discussion::Entity::find()
            .filter(entity::task_discussion::Column::TaskId.eq(i.id))
            .count(&data.sql_db)
            .await
            .map_err(ErrorInternalServerError)?;
        related_tasks.push(RelatedTask {
            id: i.id,
            name: i.name,
            finished: i.finish_date.is_some(),
            comments: count,
        });
    }
    Ok(related_tasks)
}
pub async fn handler(session: Session, data: Data<State>) -> Result<HttpResponse> {
    match session.get::<UserInfo>("user")? {
        None => Err(ErrorUnauthorized("login info not found")),
        Some(user) => {
            let tasks = get_related_tasks(&user, &data).await?;
            DashboardTemplate::new("Dashboard | Project Tachyon", user.email, tasks)
                .render_response()
                .await
        }
    }
}
