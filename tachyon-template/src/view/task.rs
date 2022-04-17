use super::{LeftBar, LeftBarItem};
use crate::view::email_hash;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "view/task.stpl")]
pub struct TaskTemplate {
    pub name: String,
}

impl LeftBar for TaskTemplate {
    const ACTIVE_ITEM: LeftBarItem = LeftBarItem::User;
}
