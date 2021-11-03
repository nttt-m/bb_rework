use crate::{
    db::DB,
    dtos::employees::{CreateEmployeeDTO, UpdateEmployeeDTO},
    middleware::sessions::Token,
    models::employees::{ActiveModel, Column, Entity, Model},
    reply::reply,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use std::convert::Infallible;
use warp::http::StatusCode;
pub async fn create(
    db: DB,
    _: Token,
    dto: CreateEmployeeDTO,
) -> Result<impl warp::Reply, Infallible> {
    let emp = ActiveModel {
        first_name: Set(dto.first_name),
        last_name: Set(dto.last_name),
        dob: Set(dto.dob),
        address: Set(dto.address),
        contact_number: Set(dto.contact_number),
        position: Set(dto.position),
        authn: Set(dto.authn),
        authz: Set(dto.authz as i8),
        pending: Set(dto.pending as i8),
        ..Default::default()
    };
    match emp.insert(db.conn.as_ref()).await {
        Ok(r) => reply(StatusCode::CREATED, Some(Ok(&r))),
        Err(e) => reply(StatusCode::INTERNAL_SERVER_ERROR, Some(Err(&e))),
    }
}
pub async fn read_one_by_id(id: i32, db: DB, _: Token) -> Result<impl warp::Reply, Infallible> {
    let emp = Entity::find_by_id(id).one(db.conn.as_ref()).await;
    match emp {
        Ok(a) => match &a {
            Some(b) => reply(StatusCode::OK, Some(Ok(&b.clone()))),
            None => reply(StatusCode::NOT_FOUND, None),
        },
        Err(e) => reply(StatusCode::INTERNAL_SERVER_ERROR, Some(Err(&e))),
    }
}

pub async fn find_by_auth(id: i32, db: DB) -> Option<Model> {
    let emp = Entity::find()
        .filter(Column::Authn.contains(&id.to_string()))
        .one(db.conn.as_ref())
        .await;
    match emp {
        Ok(e) => e,
        Err(_) => None,
    }
}

pub async fn read_all(db: DB, _: Token) -> Result<impl warp::Reply, Infallible> {
    let emp = Entity::find().all(db.conn.as_ref()).await;
    match emp {
        Ok(a) => reply(StatusCode::OK, Some(Ok(&a))),
        Err(e) => reply(StatusCode::INTERNAL_SERVER_ERROR, Some(Err(&e))),
    }
}
pub async fn update(
    id: i32,
    db: DB,
    _: Token,
    dto: UpdateEmployeeDTO,
) -> Result<impl warp::Reply, Infallible> {
    let emp = Entity::find_by_id(id).one(db.conn.as_ref()).await;
    match emp {
        Ok(a) => match &a {
            Some(b) => {
                let mut m: ActiveModel = b.to_owned().into();
                if let Some(first_name) = dto.first_name {
                    m.first_name = Set(first_name)
                }
                if let Some(last_name) = dto.last_name {
                    m.last_name = Set(last_name)
                }
                if let Some(dob) = dto.dob {
                    m.dob = Set(dob)
                }
                if let Some(address) = dto.address {
                    m.address = Set(address)
                }
                if let Some(contact_number) = dto.contact_number {
                    m.contact_number = Set(contact_number)
                }
                if let Some(position) = dto.position {
                    m.position = Set(position)
                }
                if let Some(authn) = dto.authn {
                    m.authn = Set(authn)
                }
                if let Some(authz) = dto.authz {
                    m.authz = Set(authz as i8)
                }
                if let Some(pending) = dto.pending {
                    m.pending = Set(pending as i8)
                }
                let update = m.update(db.conn.as_ref()).await;
                let res = match update {
                    Ok(m) => reply(StatusCode::OK, Some(Ok(&m))),
                    Err(e) => reply(StatusCode::INTERNAL_SERVER_ERROR, Some(Err(&e))),
                };
                res
            }
            None => reply(StatusCode::NOT_FOUND, None),
        },
        Err(e) => reply(StatusCode::INTERNAL_SERVER_ERROR, Some(Err(&e))),
    }
}
pub async fn delete(id: i32, db: DB, _: Token) -> Result<impl warp::Reply, Infallible> {
    let emp = Entity::find_by_id(id).one(db.conn.as_ref()).await;
    match emp {
        Ok(o) => {
            if let Some(model) = o {
                let amod: ActiveModel = model.into();
                match amod.delete(db.conn.as_ref()).await {
                    Ok(d) => reply(
                        StatusCode::OK,
                        Some(Ok(&format!(
                            "Deleted {} {}",
                            d.rows_affected,
                            if d.rows_affected == 1 {
                                "entry"
                            } else {
                                "entries"
                            }
                        ))),
                    ),
                    Err(e) => reply(StatusCode::INTERNAL_SERVER_ERROR, Some(Err(&e))),
                }
            } else {
                reply(StatusCode::NOT_FOUND, None)
            }
        }
        Err(e) => reply(StatusCode::INTERNAL_SERVER_ERROR, Some(Err(&e))),
    }
}

// TODO: Get by name
// TODO: Get by child ids
