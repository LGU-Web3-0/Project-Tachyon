mod dashboard;
mod object;
mod user;
pub use dashboard::*;
use md5::Digest;
pub use object::*;
pub use user::*;
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum LeftBarItem {
    Dashboard,
    Projects,
    MyTasks,
    Calendar,
    User,
    Objects,
    Settings,
}

trait LeftBar {
    const ACTIVE_ITEM: LeftBarItem;
    fn get_attributes(item: LeftBarItem) -> &'static str {
        if Self::ACTIVE_ITEM == item {
            "leftbar-active"
        } else {
            "leftbar-inactive"
        }
    }
}

fn email_hash<S: AsRef<str>>(email: S) -> String {
    let mut email_hasher = md5::Md5::new();
    email_hasher.update(email.as_ref().as_bytes());
    format!("{:x}", email_hasher.finalize())
}
