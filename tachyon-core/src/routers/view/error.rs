use crate::utils::{IntoAnyhow, LoggedUnwrap};
use actix_web::body::MessageBody;
use actix_web::dev::ServiceResponse;
use actix_web::http::Method;
use actix_web::middleware::ErrorHandlerResponse;
use actix_web::{dev, Result};
use tachyon_template::{ErrorTemplate, TemplateOnce};

pub fn error_handler<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>>
where
    B: MessageBody + 'static,
{
    if res.request().method() != Method::GET
        || res
            .response()
            .headers()
            .get("content-type")
            .and_then(|x| x.to_str().ok())
            .map(|x| x.contains("application/json") || x.contains("application/javascript"))
            .unwrap_or(false)
    {
        return Ok(ErrorHandlerResponse::Response(
            res.map_into_boxed_body().map_into_right_body(),
        ));
    }
    let status = res.status();
    let (req, res) = res.into_parts();
    let mut res = res.set_body(
        ErrorTemplate::new(
            format!(
                "{} | Tachyon Project",
                status.canonical_reason().unwrap_or("Unknown Error")
            ),
            status,
        )
        .render_once()
        .anyhow()
        .logged_unwrap(),
    );

    res.headers_mut().insert(
        actix_web::http::header::CONTENT_TYPE,
        actix_web::http::header::HeaderValue::from_static("text/html;charset=utf-8"),
    );

    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res))
}
