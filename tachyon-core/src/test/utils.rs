use std::io::{BufRead, Write};
use std::process::Stdio;
use std::time::Duration;

pub const DB_ADDRESS: &str = "postgres://postgres@localhost/test";
pub const REDIS_ADDRESS: &str = "redis://localhost:6379";

#[macro_export]
macro_rules! test_env {
    ($body : expr) => {{
        match env_logger::try_init_from_env("TACHYON_LOG") {
            _ => (),
        };
        let _uuid = uuid::Uuid::new_v4();
        let _state = $crate::Data::new($crate::State::mocked(_uuid).await.unwrap());
        let _session = actix_session::storage::RedisSessionStore::new($crate::test::REDIS_ADDRESS)
            .await
            .unwrap();
        #[allow(unused_variables)]
        let app = actix_web::test::init_service(
            actix_web::App::new()
                .wrap(
                    actix_web::middleware::ErrorHandlers::new()
                        .handler(
                            actix_web::http::StatusCode::NOT_FOUND,
                            $crate::routers::error_handler,
                        )
                        .handler(
                            actix_web::http::StatusCode::UNAUTHORIZED,
                            $crate::routers::error_handler,
                        )
                        .handler(
                            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                            $crate::routers::error_handler,
                        )
                        .handler(
                            actix_web::http::StatusCode::FORBIDDEN,
                            $crate::routers::error_handler,
                        )
                        .handler(
                            actix_web::http::StatusCode::BAD_REQUEST,
                            $crate::routers::error_handler,
                        ),
                )
                .wrap(actix_web::middleware::Compress::default())
                .wrap(actix_web::middleware::Logger::default())
                .wrap(
                    actix_session::SessionMiddleware::builder(_session, _state.key.clone())
                        .cookie_name("tachyon_id".to_string())
                        .cookie_http_only(true)
                        .cookie_same_site($crate::SameSite::Strict)
                        .build(),
                )
                .wrap($crate::CORSConfig::accept_all().middleware())
                .app_data(_state.clone())
                .service($crate::routers::routers("../static")),
        )
        .await;
        $body(app).await;
        match std::fs::remove_dir_all(format!("/tmp/tachyon-mock-test-{}", _uuid)) {
            _ => (),
        }
    }};
}

pub struct GPGHelper {
    fingerprint: String,
}

impl GPGHelper {
    pub fn new() -> Self {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(
            r#"Key-Type: default
Subkey-Type: default
Key-Usage: encrypt,sign,auth
Name-Real: Tachyon User
Name-Email: tachyon@example.com
Expire-Date: 0
%no-protection
%commit
"#
            .as_bytes(),
        )
        .unwrap();
        file.flush().unwrap();
        std::process::Command::new("gpg")
            .arg("--batch")
            .arg("--gen-key")
            .arg(file.path())
            .status()
            .unwrap();
        std::thread::sleep(Duration::from_secs(1));
        let fingerprints = std::process::Command::new("gpg")
            .arg("--with-colons")
            .arg("--fingerprint")
            .arg("tachyon@example.com")
            .output()
            .unwrap();
        let reader = std::io::BufReader::new(fingerprints.stdout.as_slice());
        let mut lines = reader.lines();
        let mut fingerprint = String::new();
        while let Some(Ok(line)) = lines.next() {
            if line.starts_with("fpr") {
                fingerprint = line.trim_start_matches("fpr").trim_matches(':').to_string();
                break;
            }
        }
        println!("created GPG key: {}", fingerprint);
        Self { fingerprint }
    }

    pub fn armored_public_key(&self) -> String {
        let output = std::process::Command::new("gpg")
            .arg("--armor")
            .arg("--export")
            .arg(&self.fingerprint)
            .output()
            .unwrap();
        String::from_utf8(output.stdout).unwrap()
    }

    pub fn signature<S: AsRef<[u8]>>(&self, msg: S) -> String {
        let child = std::process::Command::new("gpg")
            .arg("--default-key")
            .arg(&self.fingerprint)
            .arg("--detach-sign")
            .arg("--armor")
            .arg("-z")
            .arg("0")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .as_ref()
            .unwrap()
            .write_all(msg.as_ref())
            .unwrap();
        let output = child.wait_with_output().unwrap();
        String::from_utf8(output.stdout).unwrap().to_string()
    }
}

impl Drop for GPGHelper {
    fn drop(&mut self) {
        std::process::Command::new("gpg")
            .arg("--batch")
            .arg("--yes")
            .arg("--delete-secret-and-public-key")
            .arg(&self.fingerprint)
            .status()
            .unwrap();
        println!("deleted GPG key: {}", self.fingerprint);
    }
}

mod test {
    #[actix_rt::test]
    #[serial_test::serial]
    #[cfg_attr(miri, ignore)]
    async fn it_starts_up() {
        test_env!(|_| async {})
    }

    #[test]
    #[serial_test::serial]
    #[cfg_attr(miri, ignore)]
    fn it_creates_gpg() {
        use super::*;
        let helper = GPGHelper::new();
        let model =
            entity::user::Model::prepare("1", "a@b.c", "123456", helper.armored_public_key())
                .unwrap();
        let model = entity::user::Model {
            id: 0,
            name: model.name.unwrap(),
            email: model.email.unwrap(),
            password: model.password.unwrap(),
            pgp_key: model.pgp_key.unwrap(),
            wrong_pass_attempt: 0,
        };
        println!("{}", helper.armored_public_key());
        assert!(model
            .verify_signature(helper.signature("123"), "123")
            .unwrap())
    }

    #[actix_rt::test]
    #[serial_test::serial]
    #[cfg_attr(miri, ignore)]
    async fn it_handles_error_properly() {
        test_env!(|app| async move {
            let req = actix_web::test::TestRequest::post()
                .uri("/api/user/login")
                .append_header(("content-type", "application/json"))
                .set_payload(
                    r#"
                    {
                        "email" : "i@zhuyi.fan",
                        "password" : "123456"
                    }
                 "#,
                )
                .to_request();
            let res = actix_web::test::call_and_read_body(&app, req).await;
            let res = String::from_utf8(res.to_vec()).unwrap();
            assert!(res.contains(r#""success":false"#));
            let req = actix_web::test::TestRequest::get()
                .uri("/api/user/login")
                .to_request();
            let res = actix_web::test::call_and_read_body(&app, req).await;
            let res = String::from_utf8(res.to_vec()).unwrap();
            assert!(res.contains("404"));
            assert!(res.contains("Oops!"));
            assert!(res.contains("We are unable to handle your request"));

            let req = actix_web::test::TestRequest::get()
                .uri("/static/this-resource-does-not-exists.jpg")
                .to_request();
            let res: actix_web::dev::ServiceResponse<_> =
                actix_web::test::call_service(&app, req).await;
            assert_eq!(res.status(), actix_web::http::StatusCode::FORBIDDEN);
            let res = actix_web::test::read_body(res).await;
            let res = String::from_utf8(res.to_vec()).unwrap();
            assert!(res.contains("403"));
            assert!(res.contains("Oops!"));
            assert!(res.contains("We are unable to handle your request"));

            let req = actix_web::test::TestRequest::get()
                .uri("/static/")
                .to_request();
            let res: actix_web::dev::ServiceResponse<_> =
                actix_web::test::call_service(&app, req).await;
            assert_eq!(res.status(), actix_web::http::StatusCode::FORBIDDEN);
            let res = actix_web::test::read_body(res).await;
            let res = String::from_utf8(res.to_vec()).unwrap();
            assert!(res.contains("403"));
            assert!(res.contains("Oops!"));
            assert!(res.contains("We are unable to handle your request"));

            let req = actix_web::test::TestRequest::get()
                .uri("/static/logo/logo.jpeg")
                .to_request();
            let res: actix_web::dev::ServiceResponse<_> =
                actix_web::test::call_service(&app, req).await;
            assert_eq!(res.status(), actix_web::http::StatusCode::OK);
        })
    }
}
