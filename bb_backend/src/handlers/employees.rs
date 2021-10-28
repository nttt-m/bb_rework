use crate::{
    db::DB,
    dtos::employees::{CreateEmployeeDTO, UpdateEmployeeDTO},
    middleware::sessions::Token,
    models::employees::{ActiveModel, Entity},
    reply::reply,
};
use sea_orm::{Set, ActiveModelTrait, EntityTrait};
use std::convert::Infallible;
use warp::http::StatusCode;
pub async fn create(db: DB, _: Token, dto: CreateEmployeeDTO) -> Result<impl warp::Reply, Infallible> {
    let emp = ActiveModel {
        first_name: Set(dto.first_name),
        last_name: Set(dto.last_name),
        position: Set(dto.position),
        address: Set(dto.address),
        auth: Set(dto.auth),
        contact_number: Set(dto.contact_number),
        ..Default::default()
    };
    match emp.insert(db.conn.as_ref()).await {
        Ok(r) => {
            reply(StatusCode::CREATED, Some(Ok(&r)))
        },
        Err(e) => {
            reply(StatusCode::INTERNAL_SERVER_ERROR, Some(Err(&e)))
        }
    }
}
pub async fn read_one_by_id(id: i32, db: DB, _: Token) -> Result<impl warp::Reply, Infallible> {
    let emp = Entity::find_by_id(id).one(db.conn.as_ref()).await;
    match emp {
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
    let emp = Entity::find().all(db.conn.as_ref()).await;
    match emp {
        Ok(a) => {
            reply(StatusCode::OK, Some(Ok(&a)))
        }
        Err(e) => reply(StatusCode::INTERNAL_SERVER_ERROR, Some(Err(&e)))
    }
}
pub async fn update(id: i32, db: DB, _: Token, dto: UpdateEmployeeDTO) -> Result<impl warp::Reply, Infallible> {
    let emp = Entity::find_by_id(id).one(db.conn.as_ref()).await;
    match emp {
        Ok(a) => {
            match &a {
                Some(b) => {
                    let mut m: ActiveModel = b.to_owned().into();
                    if let Some(first_name) = dto.first_name {
                        m.first_name = Set(first_name)
                    }
                    if let Some(last_name) = dto.last_name {
                        m.last_name = Set(last_name)
                    }
                    if let Some(position) = dto.position {
                        m.position = Set(position)
                    }
                    if let Some(address) = dto.address {
                        m.address = Set(address)
                    }
                    if let Some(auth) = dto.auth {
                        m.auth = Set(auth)
                    }
                    if let Some(contact_number) = dto.contact_number {
                        m.contact_number = Set(contact_number)
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
    let emp = Entity::find_by_id(id).one(db.conn.as_ref()).await;
    match emp {
        Ok(o) => {
            if let Some(model) = o {
                let amod: ActiveModel = model.into();
                match amod.delete(db.conn.as_ref()).await {
                    Ok(d) => reply(StatusCode::OK, Some(Ok(&format!("Deleted {} {}", d.rows_affected, if d.rows_affected == 1 {"entry"} else {"entries"})))),
                    Err(e) => reply(StatusCode::INTERNAL_SERVER_ERROR, Some(Err(&e)))
                }
            } else {
                reply(StatusCode::NOT_FOUND, None)
            }
        }
        Err(e) => reply(StatusCode::INTERNAL_SERVER_ERROR, Some(Err(&e)))
    }
}

// TODO: Get by name
// TODO: Get by child ids