use crate::db::DB;
use crate::dtos::employees::{CreateEmployeeDTO, UpdateEmployeeDTO};
use crate::handlers::employees;
use crate::middleware::{sessions::with_authn, data::{json_body, with_db}};
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
        .and_then(employees::create)
}

fn read_one_by_id(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("employees" / i32)
        .and(warp::path::end())
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(with_authn(db))
        .and_then(employees::read_one_by_id)
}

fn read_all(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("employees")
        .and(warp::path::end())
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(with_authn(db))
        .and_then(employees::read_all)
}

fn update(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("employees" / i32)
        .and(warp::path::end())
        .and(warp::patch())
        .and(with_db(db.clone()))
        .and(with_authn(db))
        .and(json_body::<UpdateEmployeeDTO>())
        .and_then(employees::update)
}

fn delete(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("employees" /i32)
        .and(warp::path::end())
        .and(warp::delete())
        .and(with_db(db.clone()))
        .and(with_authn(db))
        .and_then(employees::delete)
}