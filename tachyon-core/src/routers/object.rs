use crate::State;
use actix_web::http::header::ContentType;
use actix_web::web::Bytes;
use actix_web::{error, web, HttpResponse, Result};
use entity::sea_orm::EntityTrait;
use sled::IVec;
use std::pin::Pin;
use std::task::{Context, Poll};
use uuid::Uuid;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ObjectRequest {
    uuid: Uuid,
}

struct ObjectData {
    inner: Option<IVec>,
}

impl futures::Stream for ObjectData {
    type Item = Result<Bytes>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        if this.inner.is_none() {
            return Poll::Ready(None);
        }
        unsafe {
            let length = this.inner.as_ref().unwrap_unchecked().len();
            match this.inner.as_ref().unwrap_unchecked().chunks(512).next() {
                None => Poll::Ready(None),
                Some(x) => {
                    let result = Poll::Ready(Some(Ok(Bytes::copy_from_slice(x))));
                    if x.len() < length {
                        let slice = this
                            .inner
                            .as_ref()
                            .unwrap_unchecked()
                            .subslice(x.len(), length - x.len());
                        this.inner.replace(slice);
                    } else {
                        this.inner = None;
                    }
                    result
                }
            }
        }
    }
}

pub async fn get_handler(
    info: web::Query<ObjectRequest>,
    data: web::Data<State>,
) -> Result<HttpResponse> {
    // TODO: auth
    let metadata: entity::object::Model = entity::object::Entity::find_by_id(info.uuid)
        .one(&data.sql_db)
        .await
        .map_err(error::ErrorNotFound)?
        .ok_or_else(|| error::ErrorNotFound("not found"))?;

    if !metadata.visibility {
        return Err(error::ErrorUnauthorized("target not authorized"));
    }

    let inner = data
        .kv_db
        .get(metadata.uuid.as_bytes())
        .map_err(error::ErrorInternalServerError)?
        .ok_or_else(|| error::ErrorNotFound("not found"))?;
    let stream = ObjectData { inner: Some(inner) };

    Ok(HttpResponse::Ok()
        .insert_header(ContentType(
            metadata
                .mimetype
                .parse()
                .map_err(error::ErrorInternalServerError)?,
        ))
        .streaming(stream))
}
