use actix_web::http::StatusCode;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "error.stpl")]
pub struct ErrorTemplate {
    pub title: String,
    pub status_code: StatusCode,
}

impl ErrorTemplate {
    pub fn new(title: String, status_code: StatusCode) -> Self {
        Self { title, status_code }
    }
    pub fn reason(status_code: StatusCode) -> &'static str {
        status_code.canonical_reason().unwrap_or("Unknown Error")
    }
}
