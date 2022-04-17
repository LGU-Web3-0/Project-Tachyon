use super::{LeftBar, LeftBarItem};
use crate::view::email_hash;
use sailfish::TemplateOnce;

pub struct TaskItem {
    pub id: i64,
    pub name: String,
    pub email_h: String,
}

impl TaskItem {
    pub fn new(id: i64, email: String, name: String) -> Self {
        let email_h = email_hash(&email);
        Self { id, name, email_h }
    }
}

#[derive(TemplateOnce)]
#[template(path = "view/task.stpl")]
pub struct TaskTemplate {
    pub title: String,
    pub email_hash: String,
    pub items: Vec<TaskItem>,
    pub prev_page_url: Option<String>,
    pub next_page_url: Option<String>,
}

impl TaskTemplate {
    pub fn new<T: AsRef<str>, E: AsRef<str>>(
        title: T,
        email: E,
        items: Vec<TaskItem>,
        page_size: usize,
        prev_page: Option<usize>,
        next_page: Option<usize>,
    ) -> Self {
        let email_hash = email_hash(email);
        let prev_page_url =
            prev_page.map(|n| format!("/view/task?page_size={}&page_no={}", page_size, n));
        let next_page_url =
            next_page.map(|n| format!("/view/task?page_size={}&page_no={}", page_size, n));

        Self {
            title: title.as_ref().to_string(),
            email_hash,
            items,
            prev_page_url,
            next_page_url,
        }
    }
}

impl LeftBar for TaskTemplate {
    const ACTIVE_ITEM: LeftBarItem = LeftBarItem::MyTasks;
}
