use crate::db::DB;
use crate::dtos::telephone_numbers::{CreateTelephoneDTO, UpdateTelephoneDTO};
use crate::routes::{json_body, with_db, with_authn};
use warp::Filter;

pub fn telephone_numbers(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    create(db.clone())
    .or(read_one_by_id(db.clone()))
    .or(read_all(db.clone()))
    .or(update(db.clone()))
    .or(delete(db.clone()))
}

fn create(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("telephone")
        .and(warp::path::end())
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(with_authn(db))
        .and(json_body::<CreateTelephoneDTO>())
        .and_then(handlers::create)
}

fn read_one_by_id(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("telephone" / i32)
        .and(warp::path::end())
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(with_authn(db))
        .and_then(handlers::read_one_by_id)
}

fn read_all(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("telephone")
        .and(warp::path::end())
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(with_authn(db))
        .and_then(handlers::read_all)
}

fn update(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("telephone" / i32)
        .and(warp::path::end())
        .and(warp::patch())
        .and(with_db(db.clone()))
        .and(with_authn(db))
        .and(json_body::<UpdateTelephoneDTO>())
        .and_then(handlers::update)
}

fn delete(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("telephone" /i32)
        .and(warp::path::end())
        .and(warp::delete())
        .and(with_db(db.clone()))
        .and(with_authn(db))
        .and_then(handlers::delete)
}

mod handlers {
    use crate::dtos::telephone_numbers::{CreateTelephoneDTO, UpdateTelephoneDTO};
    use sea_orm::{Set, ActiveModelTrait, EntityTrait};
    use std::convert::Infallible;
    use crate::models::telephone_numbers::{ActiveModel, Entity};
    use crate::db::DB;
    use crate::reply::reply;
    use crate::routes::Token;
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
}
