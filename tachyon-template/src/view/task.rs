use super::{LeftBar, LeftBarItem};
use crate::view::email_hash;
use sailfish::TemplateOnce;

pub struct TaskItem {
    pub id: i64,
    pub name: String,
}

impl TaskItem {
    pub fn new(id: i64, name: String) -> Self {
        Self { id, name }
    }
}

#[derive(TemplateOnce)]
#[template(path = "view/task.stpl")]
pub struct TaskTemplate {
    pub title: String,
    pub items: Vec<TaskItem>,
    pub prev_page_url: Option<String>,
    pub next_page_url: Option<String>,
}

impl TaskTemplate {
    pub fn new<T: AsRef<str>>(
        title: T,
        items: Vec<TaskItem>,
        page_size: usize,
        prev_page: Option<usize>,
        next_page: Option<usize>,
    ) -> Self {
        let mut prev_page_url =
            prev_page.map(|n| format!("/view/task?page_size={}&page_no={}", page_size, n));
        let mut next_page_url =
            next_page.map(|n| format!("/view/task?page_size={}&page_no={}", page_size, n));

        Self {
            title: title.as_ref().to_string(),
            items,
            prev_page_url,
            next_page_url,
        }
    }
}

impl LeftBar for TaskTemplate {
    const ACTIVE_ITEM: LeftBarItem = LeftBarItem::User;
}
