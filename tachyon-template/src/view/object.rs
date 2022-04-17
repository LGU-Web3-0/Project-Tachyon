use super::{LeftBar, LeftBarItem};
use crate::view::email_hash;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "view/object.stpl")]
pub struct ObjectTemplate {
    pub title: String,
    pub email_hash: String,
}

impl ObjectTemplate {
    pub fn new<S: AsRef<str>, E: AsRef<str>>(title: S, email: E) -> Self {
        let email_hash = email_hash(email);
        Self {
            title: title.as_ref().to_string(),
            email_hash,
        }
    }
}

impl LeftBar for ObjectTemplate {
    const ACTIVE_ITEM: LeftBarItem = LeftBarItem::Objects;
}
