use super::{LeftBar, LeftBarItem};
use crate::view::email_hash;
use sailfish::TemplateOnce;

pub struct ObjectItem {
    pub name: String,
    pub uploaded_at: chrono::DateTime<chrono::Utc>,
    pub uuid: uuid::Uuid,
    pub mimetype: String,
    pub visibility: bool,
}

#[derive(TemplateOnce)]
#[template(path = "view/object.stpl")]
pub struct ObjectTemplate {
    pub is_admin: bool,
    pub title: String,
    pub email_hash: String,
    pub objects: Vec<ObjectItem>,
    pub page_number: usize,
    pub next_page_number: Option<usize>,
    pub prev_page_number: Option<usize>,
}

impl ObjectTemplate {
    pub fn new<S: AsRef<str>, E: AsRef<str>>(
        is_admin: bool,
        title: S,
        email: E,
        objects: Vec<ObjectItem>,
        page_number: usize,
        next_page_number: Option<usize>,
        prev_page_number: Option<usize>,
    ) -> Self {
        let email_hash = email_hash(email);
        Self {
            is_admin,
            title: title.as_ref().to_string(),
            email_hash,
            objects,
            page_number,
            next_page_number,
            prev_page_number,
        }
    }
}

impl LeftBar for ObjectTemplate {
    const ACTIVE_ITEM: LeftBarItem = LeftBarItem::Objects;
}
