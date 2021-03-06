//! I will talk about the UI design here.
//! Rather than using traditional separation of front-end and backend development archtecture
//! We use template engine.
//! The view api is set for trigerring the entry point of template rendering.
//! <a href="https://github.com/rust-sailfish/sailfish">Our template engine is: Sailfish</a>
//! The data flow is:
//! ```bash
//! Main.js
//!         \
//!          \
//!     RsyncRenderOnce: api for template engine
//!                      |
//!                      |
//!             Load corresponding .stpl to render
//!                and get render response
//!          /
//!         /
//!   Webpack bundle all things and send to frontend
//!
//!
//!
#![allow(clippy::too_many_arguments)]
use actix_web::error::ErrorInternalServerError;
use actix_web::{HttpResponse, Result};
use async_trait::async_trait;
pub use sailfish::*;
mod error;
mod index;
pub mod view;

pub use error::*;
pub use index::*;

#[async_trait]
pub trait AsyncRenderOnce: Sized {
    async fn render(self) -> RenderResult;
    async fn render_response(self) -> Result<HttpResponse> {
        let res = self.render().await.map_err(ErrorInternalServerError)?;
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(res))
    }
}

#[async_trait]
impl<T> AsyncRenderOnce for T
where
    T: Sync + Send + TemplateOnce,
{
    async fn render(self) -> RenderResult {
        self.render_once()
    }
}

#[async_trait]
pub trait AsyncRender {
    async fn render(&self) -> RenderResult;
    async fn render_response(&self) -> Result<HttpResponse> {
        let res = self.render().await.map_err(ErrorInternalServerError)?;
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(res))
    }
}

#[async_trait]
impl<T> AsyncRender for T
where
    T: Sync + Send + Template,
{
    async fn render(&self) -> RenderResult {
        Template::render(self)
    }
}

#[derive(TemplateOnce)]
#[template(path = "hello.stpl")]
pub struct HelloTemplate {
    messages: Vec<String>,
}

impl HelloTemplate {
    pub fn new<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Self {
        Self {
            messages: input.map(|x| x.as_ref().to_string()).collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    #[cfg_attr(miri, ignore)]
    async fn hello() -> std::result::Result<(), RenderError> {
        HelloTemplate::new(["a", "b", "c"].into_iter())
            .render()
            .await?;
        Ok(())
    }
}
