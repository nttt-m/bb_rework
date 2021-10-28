use crate::db::DB;
use crate::dtos::telephone_numbers::{CreateTelephoneDTO, UpdateTelephoneDTO};
use crate::handlers::telephone_numbers;
use crate::middleware::{sessions::with_authn, data::{json_body, with_db}};
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
        .and_then(telephone_numbers::create)
}

fn read_one_by_id(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("telephone" / i32)
        .and(warp::path::end())
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(with_authn(db))
        .and_then(telephone_numbers::read_one_by_id)
}

fn read_all(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("telephone")
        .and(warp::path::end())
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(with_authn(db))
        .and_then(telephone_numbers::read_all)
}

fn update(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("telephone" / i32)
        .and(warp::path::end())
        .and(warp::patch())
        .and(with_db(db.clone()))
        .and(with_authn(db))
        .and(json_body::<UpdateTelephoneDTO>())
        .and_then(telephone_numbers::update)
}

fn delete(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("telephone" /i32)
        .and(warp::path::end())
        .and(warp::delete())
        .and(with_db(db.clone()))
        .and(with_authn(db))
        .and_then(telephone_numbers::delete)
}