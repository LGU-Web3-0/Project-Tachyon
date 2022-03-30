use sailfish::TemplateOnce;
use async_trait::async_trait;
use actix_web::{Result, HttpResponse};
use actix_web::error::ErrorInternalServerError;
pub use sailfish::*;

#[async_trait]
pub trait AsyncRenderOnce : Sized {
    async fn render(self) -> RenderResult;
    async fn render_response(self) -> Result<HttpResponse> {
        let res = self.render().await.map_err(ErrorInternalServerError)?;
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(res)
        )
    }
}

#[async_trait]
impl<T> AsyncRenderOnce for T
    where T : Sync + Send + TemplateOnce {
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
    where T : Sync + Send + Template {
    async fn render(&self) -> RenderResult {
        Template::render(self)
    }
}

#[derive(TemplateOnce)]
#[template(path = "hello.stpl")]
pub struct HelloTemplate {
    messages: Vec<String>
}

impl HelloTemplate {
    pub fn new<S: AsRef<str>, I : Iterator<Item = S>>(input: I) -> Self {
        Self {
            messages: input.map(|x|x.as_ref().to_string()).collect()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn hello() -> std::result::Result<(), RenderError> {
        let data = HelloTemplate::new(["a", "b", "c"].into_iter()).render().await?;
        dbg!(data);
        Ok(())
    }
}