use crate::dtos::addresses::{CreateAddressDTO, UpdateAddressDTO};
use sea_orm::{Set, ActiveModelTrait, EntityTrait};
use std::convert::Infallible;
use crate::models::addresses::{ActiveModel, Entity};
use crate::db::DB;
use crate::reply::reply;
use warp::http::StatusCode;
use crate::middleware::sessions::Token;
pub async fn create(db: DB, _: Token, dto: CreateAddressDTO) -> Result<impl warp::Reply, Infallible> {
    let addr = ActiveModel {
        house_name_number: Set(dto.house_name_number),
        street: Set(dto.street),
        town_city: Set(dto.town_city),
        region: Set(dto.region),
        postal_code: Set(dto.postal_code),
        country: Set(dto.country),
        ..Default::default()
    };
    match addr.insert(db.conn.as_ref()).await {
        Ok(r) => {
            reply(StatusCode::CREATED, Some(Ok(&r)))
        },
        Err(e) => {
            reply(StatusCode::INTERNAL_SERVER_ERROR, Some(Err(&e)))
        }
    }
}
pub async fn read_one_by_id(id: i32, db: DB, _: Token) -> Result<impl warp::Reply, Infallible> {
    let addr = Entity::find_by_id(id).one(db.conn.as_ref()).await;
    match addr {
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
    let addr = Entity::find().all(db.conn.as_ref()).await;
    match addr {
        Ok(a) => {
            reply(StatusCode::OK, Some(Ok(&a)))
        }
        Err(e) => reply(StatusCode::INTERNAL_SERVER_ERROR, Some(Err(&e)))
    }
}
pub async fn update(id: i32, db: DB, _: Token, dto: UpdateAddressDTO) -> Result<impl warp::Reply, Infallible> {
    let addr = Entity::find_by_id(id).one(db.conn.as_ref()).await;
    match addr {
        Ok(a) => {
            match &a {
                Some(b) => {
                    let mut m: ActiveModel = b.to_owned().into();
                    if let Some(number) = dto.house_name_number {
                        m.house_name_number = Set(number)
                    }
                    if let Some(street) = dto.street {
                        m.street = Set(street.into())
                    }
                    if let Some(town) = dto.town_city {
                        m.town_city = Set(town.into())
                    }
                    if let Some(region) = dto.region {
                        m.region = Set(region.into())
                    }
                    if let Some(post) = dto.postal_code {
                        m.postal_code = Set(post.into())
                    }
                    if let Some(country) = dto.country {
                        m.country = Set(country.into())
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
    let addr = Entity::find_by_id(id).one(db.conn.as_ref()).await;
    match addr {
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