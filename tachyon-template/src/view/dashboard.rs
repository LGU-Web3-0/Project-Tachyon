use super::{LeftBar, LeftBarItem};
use crate::view::email_hash;
use sailfish::TemplateOnce;

pub struct RelatedTask {
    pub id: i64,
    pub name: String,
    pub finished: bool,
    pub comments: usize,
}

#[derive(TemplateOnce)]
#[template(path = "view/dashboard.stpl")]
pub struct DashboardTemplate {
    pub title: String,
    pub email_hash: String,
    pub related_tasks: Vec<RelatedTask>,
    pub total_tasks: usize,
    pub finished_tasks: usize,
    pub due_works: [usize; 6],
}

impl DashboardTemplate {
    pub fn new<S: AsRef<str>, E: AsRef<str>>(
        title: S,
        email: E,
        related_tasks: Vec<RelatedTask>,
        total_tasks: usize,
        finished_tasks: usize,
        due_works: [usize; 6],
    ) -> Self {
        let email_hash = email_hash(email);
        Self {
            title: title.as_ref().to_string(),
            email_hash,
            related_tasks,
            total_tasks,
            finished_tasks,
            due_works,
        }
    }
}

impl LeftBar for DashboardTemplate {
    const ACTIVE_ITEM: LeftBarItem = LeftBarItem::Dashboard;
}
