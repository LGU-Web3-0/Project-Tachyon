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
