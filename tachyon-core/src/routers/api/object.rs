use crate::session::UserInfo;
use crate::{IntoAnyhow, State, StatusCode};
use actix_multipart::Multipart;
use actix_session::Session;
use actix_web::error::ErrorInternalServerError;
use actix_web::http::header::{ContentDisposition, ContentType, DispositionParam, DispositionType};
use actix_web::web::Bytes;
use actix_web::{error, web, HttpResponse, Result};
use entity::sea_orm::DatabaseBackend::Postgres;
use entity::sea_orm::QueryFilter;
use entity::sea_orm::{ActiveModelTrait, ConnectionTrait, Statement};
use entity::sea_orm::{ActiveValue, ColumnTrait, EntityTrait};
use futures::{StreamExt, TryFutureExt};
use sled::IVec;
use std::pin::Pin;
use std::task::{Context, Poll};
use uuid::Uuid;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ObjectRequest {
    uuid: Option<Uuid>,
    name: Option<String>,
}

struct ObjectData {
    inner: Option<IVec>,
}

const CHUNK_SIZE: usize = 1024 * 1024;

impl futures::Stream for ObjectData {
    type Item = Result<Bytes>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        if this.inner.is_none() {
            return Poll::Ready(None);
        }
        unsafe {
            let length = this.inner.as_ref().unwrap_unchecked().len();
            match this
                .inner
                .as_ref()
                .unwrap_unchecked()
                .chunks(CHUNK_SIZE)
                .next()
            {
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

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct ObjectResult {
    success: bool,
    message: Option<String>,
}

pub async fn upload(
    session: Session,
    data: web::Data<State>,
    mut payload: Multipart,
) -> Result<HttpResponse> {
    async fn parse_data(payload: &mut Multipart) -> Result<(entity::object::ActiveModel, Vec<u8>)> {
        let mut model = entity::object::ActiveModel {
            uuid: ActiveValue::NotSet,
            name: ActiveValue::NotSet,
            visibility: ActiveValue::Set(false),
            upload_time: ActiveValue::Set(chrono::Utc::now()),
            mimetype: ActiveValue::NotSet,
        };
        let mut content = Vec::new();
        while let Some(item) = payload.next().await {
            let mut field = item?;
            match field.name() {
                "file" => {
                    while let Some(x) = field.next().await {
                        content.extend(x?);
                    }
                    model.mimetype = ActiveValue::Set(field.content_type().to_string());
                }
                "visibility" => {
                    let mut data = Vec::new();
                    while let Some(x) = field.next().await {
                        data.extend(x?);
                    }
                    log::error!("{}", String::from_utf8(data.clone()).unwrap());
                    model.visibility = ActiveValue::Set(data.as_slice() == b"on");
                }
                "filename" => {
                    let mut data = Vec::new();
                    while let Some(x) = field.next().await {
                        data.extend(x?);
                    }
                    model.name = ActiveValue::Set(
                        String::from_utf8(data).map_err(ErrorInternalServerError)?,
                    );
                }
                _ => (),
            }
        }
        Ok((model, content))
    }

    async fn insert_kv(
        mut model: entity::object::ActiveModel,
        content: Vec<u8>,
        data: &web::Data<State>,
    ) -> Result<entity::object::ActiveModel> {
        let mut uuid;
        loop {
            uuid = Uuid::new_v4();
            match data
                .kv_db
                .compare_and_swap(
                    uuid.as_bytes(),
                    Option::<&[u8]>::None,
                    Some(content.as_slice()),
                )
                .map_err(ErrorInternalServerError)
            {
                Ok(Ok(_)) => {
                    model.uuid = ActiveValue::Set(uuid);
                    break;
                }
                Ok(Err(_)) => tokio::task::yield_now().await,
                Err(e) => return Err(e),
            }
        }

        if let Err(e) = data.kv_db.flush_async().await {
            log::error!("sled insertion error: {}", e);
            return Err(ErrorInternalServerError(e));
        };

        Ok(model)
    }
    match session.get::<UserInfo>("user")? {
        None => simd_json::to_string(&ObjectResult {
            success: false,
            message: Some("unauthorized".to_string()),
        })
        .map_err(ErrorInternalServerError)
        .map(|x| {
            HttpResponse::Ok()
                .content_type("application/json")
                .status(StatusCode::UNAUTHORIZED)
                .json(ObjectResult {
                    success: false,
                    message: Some(x),
                })
        }),
        _ => Ok(parse_data(&mut payload)
            .and_then(|(model, content)| insert_kv(model, content, &data))
            .and_then(|model| model.insert(&data.sql_db).map_err(ErrorInternalServerError))
            .await
            .map(|_| {
                HttpResponse::Created()
                    .content_type("application/json")
                    .json(ObjectResult {
                        success: true,
                        message: None,
                    })
            })
            .unwrap_or_else(|e| {
                HttpResponse::BadRequest()
                    .content_type("application/json")
                    .json(ObjectResult {
                        success: false,
                        message: Some(e.to_string()),
                    })
            })),
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct VisibilityChange {
    pub uuid: Uuid,
}

pub async fn change_visibility(
    info: web::Json<VisibilityChange>,
    data: web::Data<State>,
    session: Session,
) -> Result<HttpResponse> {
    if session.get::<UserInfo>("user")?.is_none() {
        return Ok(HttpResponse::Unauthorized().json(ObjectResult {
            success: false,
            message: Some("unauthorized".to_string()),
        }));
    }
    Ok(
        match data
            .sql_db
            .execute(Statement::from_string(
                Postgres,
                format!(
                    r#"UPDATE object SET visibility = NOT visibility WHERE uuid = '{}'"#,
                    info.uuid
                ),
            ))
            .await
        {
            Ok(_) => HttpResponse::Ok().json(ObjectResult {
                success: true,
                message: None,
            }),
            Err(e) => HttpResponse::BadRequest().json(ObjectResult {
                success: false,
                message: Some(e.to_string()),
            }),
        },
    )
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct DeleteRequest {
    pub uuid: Uuid,
}

pub async fn delete(
    info: web::Json<DeleteRequest>,
    data: web::Data<State>,
    session: Session,
) -> Result<HttpResponse> {
    if session.get::<UserInfo>("user")?.is_none() {
        return Ok(HttpResponse::Unauthorized().json(ObjectResult {
            success: false,
            message: Some("unauthorized".to_string()),
        }));
    }
    Ok(
        match data
            .sql_db
            .execute(Statement::from_string(
                Postgres,
                format!(r#"DELETE FROM object WHERE uuid = '{}'"#, info.uuid),
            ))
            .await
            .anyhow()
            .and_then(|x| {
                if x.rows_affected() != 0 {
                    data.kv_db.remove(info.uuid.as_bytes()).anyhow().and(Ok(()))
                } else {
                    Ok(())
                }
            }) {
            Ok(_) => HttpResponse::Ok().json(ObjectResult {
                success: true,
                message: None,
            }),
            Err(e) => HttpResponse::BadRequest().json(ObjectResult {
                success: false,
                message: Some(e.to_string()),
            }),
        },
    )
}

pub async fn get_handler(
    info: web::Query<ObjectRequest>,
    data: web::Data<State>,
) -> Result<HttpResponse> {
    let metadata: entity::object::Model = if let Some(uuid) = info.uuid {
        entity::object::Entity::find_by_id(uuid)
            .one(&data.sql_db)
            .await
            .map_err(error::ErrorNotFound)?
            .ok_or_else(|| error::ErrorNotFound("not found"))?
    } else if let Some(name) = &info.name {
        entity::object::Entity::find()
            .filter(entity::object::Column::Name.eq(name.as_str()))
            .one(&data.sql_db)
            .await
            .map_err(error::ErrorNotFound)?
            .ok_or_else(|| error::ErrorNotFound("not found"))?
    } else {
        return Err(error::ErrorBadRequest("invalid request"));
    };

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
        .insert_header(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![DispositionParam::Filename(
                metadata.name.as_str().to_string(),
            )],
        })
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
