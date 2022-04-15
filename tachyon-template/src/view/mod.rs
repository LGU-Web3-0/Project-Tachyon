mod dashboard;
pub use dashboard::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum LeftBarItem {
    Dashboard,
    Projects,
    MyTasks,
    Calendar,
    TimeManage,
    Reports,
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
