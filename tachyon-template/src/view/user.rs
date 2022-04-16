use super::{LeftBar, LeftBarItem};
use crate::view::email_hash;
use sailfish::TemplateOnce;

pub struct UserItem {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub email_hash: String,
    pub is_locked: bool,
    pub fingerprint: String,
}

impl UserItem {
    pub fn new(id: i64, email: String, name: String, is_locked: bool, fingerprint: String) -> Self {
        let email_hash = email_hash(&email);
        Self {
            id,
            email,
            name,
            email_hash,
            is_locked,
            fingerprint,
        }
    }
}

#[derive(TemplateOnce)]
#[template(path = "view/user.stpl")]
pub struct UserTemplate {
    pub title: String,
    pub email_hash: String,
    pub items: Vec<UserItem>,
    pub prev_page_url: Option<String>,
    pub next_page_url: Option<String>,
}

impl UserTemplate {
    pub fn new<S: AsRef<str>, E: AsRef<str>>(
        title: S,
        email: E,
        items: Vec<UserItem>,
        page_size: usize,
        prev_page: Option<usize>,
        next_page: Option<usize>,
        search_string: Option<String>,
    ) -> Self {
        let email_hash = email_hash(email);
        let mut prev_page_url =
            prev_page.map(|n| format!("/view/user?page_size={}&page_no={}", page_size, n));
        let mut next_page_url =
            next_page.map(|n| format!("/view/user?page_size={}&page_no={}", page_size, n));
        if let Some(data) = search_string {
            let encoded = urlencoding::encode(&data);
            prev_page_url = prev_page_url.map(|mut url| {
                url.push_str("&search_string=");
                url.push_str(&encoded);
                url
            });
            next_page_url = next_page_url.map(|mut url| {
                url.push_str("&search_string=");
                url.push_str(&encoded);
                url
            });
        }
        Self {
            title: title.as_ref().to_string(),
            email_hash,
            items,
            prev_page_url,
            next_page_url,
        }
    }
}

impl LeftBar for UserTemplate {
    const ACTIVE_ITEM: LeftBarItem = LeftBarItem::User;
}
