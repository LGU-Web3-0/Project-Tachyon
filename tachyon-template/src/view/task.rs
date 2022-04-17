use super::{LeftBar, LeftBarItem};
use crate::view::email_hash;
use sailfish::runtime::{Buffer, Render};
use sailfish::{RenderError, TemplateOnce};

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
<<<<<<< HEAD
        let email_hash = email_hash(email);
        let prev_page_url =
=======
        let mut prev_page_url =
>>>>>>> 18dbd10 (feat: add UI for task board)
            prev_page.map(|n| format!("/view/task?page_size={}&page_no={}", page_size, n));
        let mut next_page_url =
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

pub struct UserData {
    name: String,
    email: String,
    email_hash: String,
}

impl UserData {
    pub fn new<N: AsRef<str>, S: AsRef<str>>(name: N, email: S) -> Self {
        Self {
            name: name.as_ref().to_string(),
            email: email.as_ref().to_string(),
            email_hash: email_hash(email),
        }
    }
}

pub struct Comment {
    pub id: i64,
    pub content: RawString,
    pub update_time: chrono::DateTime<chrono::Utc>,
    pub author: UserData,
}

impl Comment {
    pub fn new<S: AsRef<str>>(
        id: i64,
        raw: S,
        update_time: chrono::DateTime<chrono::Utc>,
        author: UserData,
    ) -> Self {
        let opt = pulldown_cmark::Options::all();
        let parser = pulldown_cmark::Parser::new_ext(raw.as_ref(), opt);
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, parser);
        Self {
            id,
            content: RawString(ammonia::clean(&html)),
            update_time,
            author,
        }
    }
}

#[derive(TemplateOnce)]
#[template(path = "view/task_detail.stpl")]
pub struct TaskDetailTemplate {
    title: String,
    email_hash: String,
    email: String,
    task_id: i64,
    name: String,
    created_at: chrono::DateTime<chrono::Utc>,
    finished_at: Option<chrono::DateTime<chrono::Utc>>,
    assigned_users: Vec<UserData>,
    comments: Vec<Comment>,
    description: String,
}

pub struct RawString(String);

impl Render for RawString {
    fn render(&self, b: &mut Buffer) -> Result<(), RenderError> {
        self.0.render(b)
    }

    fn render_escaped(&self, b: &mut Buffer) -> Result<(), RenderError> {
        self.0.render(b)
    }
}

impl TaskDetailTemplate {
    #[allow(clippy::too_many_arguments)]
    pub fn new<T: AsRef<str>, E: AsRef<str>, N: AsRef<str>, D: AsRef<str>>(
        title: T,
        email: E,
        task_id: i64,
        name: N,
        created_at: chrono::DateTime<chrono::Utc>,
        finished_at: Option<chrono::DateTime<chrono::Utc>>,
        assigned_users: Vec<UserData>,
        comments: Vec<Comment>,
        description: D,
    ) -> Self {
        TaskDetailTemplate {
            title: title.as_ref().to_string(),
            email_hash: email_hash(email.as_ref()),
            email: email.as_ref().to_string(),
            task_id,
            name: name.as_ref().to_string(),
            created_at,
            finished_at,
            assigned_users,
            comments,
            description: description.as_ref().to_string(),
        }
    }
}

impl LeftBar for TaskDetailTemplate {
    const ACTIVE_ITEM: LeftBarItem = LeftBarItem::MyTasks;
}
