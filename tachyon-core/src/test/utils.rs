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
                .wrap(actix_web::middleware::Logger::default())
                .wrap(
                    actix_session::SessionMiddleware::builder(_session, _state.key.clone())
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

mod test {
    #[actix_rt::test]
    #[serial_test::serial]
    #[cfg_attr(miri, ignore)]
    async fn it_starts_up() {
        test_env!(|_| async {})
    }
}
