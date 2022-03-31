use crate::{startup, CORSConfig, State};
use actix_session::storage::CookieSessionStore;
use actix_web::web::Data;
pub use serial_test::serial;
use std::env::current_exe;
use std::process::Child;
use std::sync::Arc;

pub const ADDRESS: &str = "0.0.0.0:8000";

pub async fn startup_mock_app(uuid: uuid::Uuid) {
    let state = Data::new(State::mocked(uuid).unwrap());
    let cors_config = Some(CORSConfig::accept_all());

    startup(
        "error",
        "../static".parse().unwrap(),
        Arc::new(move || state.clone()),
        Arc::new(move || cors_config.clone()),
        Arc::new(move || CookieSessionStore::default()),
        "0.0.0.0:8000",
    )
    .await
    .unwrap();
}

pub struct ChildProcess(Child, uuid::Uuid);

pub async fn startup_background() -> ChildProcess {
    let uuid = uuid::Uuid::new_v4();
    let child = std::process::Command::new(current_exe().unwrap())
        .arg("internal_startup_hook")
        .env("TACHYON_BACKGROUND", format!("{}", uuid))
        .spawn()
        .unwrap();
    awc::Client::default()
        .get(format!("http://{}/", ADDRESS))
        .send()
        .await
        .unwrap();
    ChildProcess(child, uuid)
}

impl Drop for ChildProcess {
    fn drop(&mut self) {
        self.0.kill().unwrap();
        match std::fs::remove_dir_all(format!("/tmp/tachyon-mock-test-{}", self.1)) {
            _ => (),
        }
    }
}

mod test {
    use crate::test::{serial, startup_background, startup_mock_app};

    #[actix_rt::test]
    #[serial]
    async fn it_starts_up1() {
        let _child = startup_background().await;
    }

    #[actix_rt::test]
    async fn internal_startup_hook() {
        if let Ok(uuid) =
            std::env::var("TACHYON_BACKGROUND").map(|x| uuid::Uuid::parse_str(&x).unwrap())
        {
            startup_mock_app(uuid).await;
        }
    }
}
