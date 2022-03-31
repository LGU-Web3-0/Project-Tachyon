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
    #[crate::test::serial]
    async fn it_gives_hello() {
        use crate::routers::api::hello_world::HelloWorld;
        use crate::test::{startup_background, ADDRESS};
        use awc::Client;

        let _child = startup_background().await;
        let client = Client::default();
        let res: HelloWorld = client
            .get(format!("http://{}/api/hello", ADDRESS))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        assert_eq!(res.message, "123")
    }
}
