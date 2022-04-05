use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Permissions {
    pub task_management: bool,
    pub file_management: bool,
    pub team_management: bool,
    pub user_management: bool,
    pub system_management: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub perms: Permissions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordChange {
    pub email: String,
    pub token: Uuid,
}
