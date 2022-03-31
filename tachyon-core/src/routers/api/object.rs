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

#[cfg(test)]
mod test {

    #[cfg(all(not(miri), test))]
    #[actix_rt::test]
    async fn it_polls_fully() {
        use crate::routers::api::object::ObjectData;
        use futures::StreamExt;
        use rand::distributions::Alphanumeric;
        use rand::prelude::*;

        let uuid = uuid::Uuid::new_v4();
        let db = sled::open(format!("/tmp/tachyon-ut-{}", uuid)).unwrap();

        for i in [1, 2, 123, 555, 5261, 114514, 1024000] {
            let rand_string: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(i)
                .map(char::from)
                .collect();

            db.insert(format!("test-{}", i), rand_string.as_bytes())
                .unwrap();
            let data = db.get(format!("test-{}", i)).unwrap().unwrap();
            let data = ObjectData { inner: Some(data) };
            let data: Vec<u8> = data
                .collect::<Vec<_>>()
                .await
                .into_iter()
                .filter_map(Result::ok)
                .map(|x| x.to_vec())
                .flatten()
                .collect();
            assert_eq!(data, rand_string.as_bytes())
        }

        std::fs::remove_dir_all(format!("/tmp/tachyon-ut-{}", uuid)).unwrap();
    }
}
