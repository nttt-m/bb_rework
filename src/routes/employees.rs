use crate::db::DB;
use crate::dtos::employees::{CreateEmployeeDTO, UpdateEmployeeDTO};
use crate::routes::{json_body, with_db, with_authn};
use warp::Filter;

pub fn employees(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    create(db.clone())
    .or(read_one_by_id(db.clone()))
    .or(read_all(db.clone()))
    .or(update(db.clone()))
    .or(delete(db.clone()))
}

fn create(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("employees")
        .and(warp::path::end())
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(with_authn(db))
        .and(json_body::<CreateEmployeeDTO>())
        .and_then(handlers::create)
}

fn read_one_by_id(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("employees" / i32)
        .and(warp::path::end())
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(with_authn(db))
        .and_then(handlers::read_one_by_id)
}

fn read_all(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("employees")
        .and(warp::path::end())
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(with_authn(db))
        .and_then(handlers::read_all)
}

fn update(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("employees" / i32)
        .and(warp::path::end())
        .and(warp::patch())
        .and(with_db(db.clone()))
        .and(with_authn(db))
        .and(json_body::<UpdateEmployeeDTO>())
        .and_then(handlers::update)
}

fn delete(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("employees" /i32)
        .and(warp::path::end())
        .and(warp::delete())
        .and(with_db(db.clone()))
        .and(with_authn(db))
        .and_then(handlers::delete)
}

mod handlers {
    use crate::dtos::employees::{CreateEmployeeDTO, UpdateEmployeeDTO};
    use sea_orm::{Set, ActiveModelTrait, EntityTrait};
    use std::convert::Infallible;
    use crate::models::employees::{ActiveModel, Entity};
    use crate::db::DB;
    use crate::reply::reply;
    use crate::routes::Token;
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
}
