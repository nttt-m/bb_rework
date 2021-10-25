use crate::dtos::authentication::{CreateAuthDTO, UpdateAuthDTO};
use sea_orm::{Set, ActiveModelTrait, EntityTrait};
use std::convert::Infallible;
use crate::models::authentication::{ActiveModel, Entity};
use crate::db::DB;
use crate::reply::reply;
use crate::middleware::sessions::Token;
use warp::http::StatusCode;
pub async fn create(db: DB, _: Token, dto: CreateAuthDTO) -> Result<impl warp::Reply, Infallible> {
    let auth = ActiveModel {
        username: Set(dto.username),
        password: Set(dto.password),
        ..Default::default()
    };
    match auth.insert(db.conn.as_ref()).await {
        Ok(r) => {
            reply(StatusCode::CREATED, Some(Ok(&r)))
        },
        Err(e) => {
            reply(StatusCode::INTERNAL_SERVER_ERROR, Some(Err(&e)))
        }
    }
}
pub async fn read_one_by_id(id: i32, db: DB, _: Token) -> Result<impl warp::Reply, Infallible> {
    let auth = Entity::find_by_id(id).one(db.conn.as_ref()).await;
    match auth {
        Ok(a) => {
            match &a {
                Some(b) => {
                    reply(StatusCode::OK, Some(Ok(&b.clone())))
                },
                None => {
                    reply(StatusCode::NOT_FOUND, None)
                }
            }
        }
        Err(e) => {
            reply(StatusCode::INTERNAL_SERVER_ERROR, Some(Err(&e)))
        }
    }
}
pub async fn read_all(db: DB, _: Token) -> Result<impl warp::Reply, Infallible> {
    let auth = Entity::find().all(db.conn.as_ref()).await;
    match auth {
        Ok(a) => {
            reply(StatusCode::OK, Some(Ok(&a)))
        }
        Err(e) => reply(StatusCode::INTERNAL_SERVER_ERROR, Some(Err(&e)))
    }
}
pub async fn update(id: i32, db: DB, _: Token, dto: UpdateAuthDTO) -> Result<impl warp::Reply, Infallible> {
    let auth = Entity::find_by_id(id).one(db.conn.as_ref()).await;
    match auth {
        Ok(a) => {
            match &a {
                Some(b) => {
                    let mut m: ActiveModel = b.to_owned().into();
                    if let Some(username) = dto.username {
                        m.username = Set(username)
                    }
                    if let Some(password) = dto.password {
                        m.password = Set(password)
                    }
                    let update = m.update(db.conn.as_ref()).await;
                    let res = match update {
                        Ok(m) => reply(StatusCode::OK, Some(Ok(&m))),
                        Err(e) => reply(StatusCode::INTERNAL_SERVER_ERROR, Some(Err(&e)))
                    };
                    res
                },
                None => reply(StatusCode::NOT_FOUND, None)
            }
        }
        Err(e) => reply(StatusCode::INTERNAL_SERVER_ERROR, Some(Err(&e)))
    }
}
pub async fn delete(id: i32, db: DB, _: Token) -> Result<impl warp::Reply, Infallible> {
    let auth = Entity::find_by_id(id).one(db.conn.as_ref()).await;
    match auth {
        Ok(o) => {
            if let Some(model) = o {
                let amod: ActiveModel = model.into();
                match amod.delete(db.conn.as_ref()).await {
                    Ok(d) => reply(StatusCode::OK, Some(Ok(&format!("Deleted {} entry", d.rows_affected)))),
                    Err(e) => reply(StatusCode::INTERNAL_SERVER_ERROR, Some(Err(&e)))
                }
            } else {
                reply(StatusCode::NOT_FOUND, None)
            }
        }
        Err(e) => reply(StatusCode::INTERNAL_SERVER_ERROR, Some(Err(&e)))
    }
}