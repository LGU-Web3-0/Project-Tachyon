use super::{LeftBar, LeftBarItem};
use md5::Digest;
use sailfish::TemplateOnce;
#[derive(TemplateOnce)]
#[template(path = "view/dashboard.stpl")]
pub struct DashboardTemplate {
    pub title: String,
    pub email_hash: String,
}

impl DashboardTemplate {
    pub fn new<S: AsRef<str>, E: AsRef<str>>(title: S, email: E) -> Self {
        let mut email_hasher = md5::Md5::new();
        email_hasher.update(email.as_ref().as_bytes());
        let email_hash = format!("{:x}", email_hasher.finalize());
        Self {
            title: title.as_ref().to_string(),
            email_hash,
        }
    }
}

impl LeftBar for DashboardTemplate {
    const ACTIVE_ITEM: LeftBarItem = LeftBarItem::Dashboard;
}
