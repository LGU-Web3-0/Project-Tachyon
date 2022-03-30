use actix_web::{HttpResponse, Result};
use tachyon_template::{AsyncRenderOnce, HelloTemplate};

pub async fn handler() -> Result<HttpResponse> {
    HelloTemplate::new(["a", "b", "c"].into_iter())
        .render_response()
        .await
}