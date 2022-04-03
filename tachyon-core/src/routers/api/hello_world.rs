use actix_web::web::Json;
use chrono::Utc;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct HelloWorld {
    message: String,
    time: chrono::DateTime<Utc>,
}

pub async fn handler() -> Json<HelloWorld> {
    Json(HelloWorld {
        message: "123".to_string(),
        time: chrono::Utc::now(),
    })
}

#[cfg(test)]
mod test {
    #[cfg(all(not(miri), test, feature = "integration-test"))]
    #[actix_rt::test]
    #[serial_test::serial]
    async fn it_gives_hello() {
        use crate::routers::api::hello_world::HelloWorld;
        use actix_web::test;
        crate::test_env!(|app| async move {
            let req = test::TestRequest::get().uri("/api/hello").to_request();
            let resp: HelloWorld = test::call_and_read_body_json(&app, req).await;
            assert_eq!(resp.message, "123")
        });
    }
}
