use crate::{
    db::DB,
    dtos::telephone_numbers::{CreateTelephoneDTO, UpdateTelephoneDTO},
    middleware::sessions::Token,
    models::telephone_numbers::{ActiveModel, Entity},
    reply::reply
};
use sea_orm::{Set, ActiveModelTrait, EntityTrait};
use std::convert::Infallible;
use warp::http::StatusCode;
pub async fn create(db: DB, _: Token, dto: CreateTelephoneDTO) -> Result<impl warp::Reply, Infallible> {
    let tele = ActiveModel {
        telephone_number: Set(dto.telephone_number),
        ..Default::default()
    };
    match tele.insert(db.conn.as_ref()).await {
        Ok(r) => {
            reply(StatusCode::CREATED, Some(Ok(&r)))
        },
        Err(e) => {
            reply(StatusCode::INTERNAL_SERVER_ERROR, Some(Err(&e)))
        }
    }
}
pub async fn read_one_by_id(id: i32, db: DB, _: Token) -> Result<impl warp::Reply, Infallible> {
    let tele = Entity::find_by_id(id).one(db.conn.as_ref()).await;
    match tele {
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
    let tele = Entity::find().all(db.conn.as_ref()).await;
    match tele {
        Ok(a) => {
            reply(StatusCode::OK, Some(Ok(&a)))
        }
        Err(e) => reply(StatusCode::INTERNAL_SERVER_ERROR, Some(Err(&e)))
    }
}
pub async fn update(id: i32, db: DB, _: Token, dto: UpdateTelephoneDTO) -> Result<impl warp::Reply, Infallible> {
    let tele = Entity::find_by_id(id).one(db.conn.as_ref()).await;
    match tele {
        Ok(a) => {
            match &a {
                Some(b) => {
                    let mut m: ActiveModel = b.to_owned().into();
                    if let Some(telephone_number) = dto.telephone_number {
                        m.telephone_number = Set(telephone_number)
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
    let tele = Entity::find_by_id(id).one(db.conn.as_ref()).await;
    match tele {
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

// TODO: Get by number