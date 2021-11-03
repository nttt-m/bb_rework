use crate::handlers::core::{
    handle_client_hmac, handle_client_pubkey, start_login, PubKeyDTO, VerifyDTO,
};
use crate::{
    db::DB,
    dtos::core::{LoginDTO, RegisterDTO},
    middleware::data::{json_body, with_db},
};
use warp::Filter;

pub fn core(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // register(db.clone())
    //     .and(login(db.clone()));
    get_login(db)
}

fn register(db: DB) {
    warp::path!("register")
        .and(warp::path::end())
        .and(warp::post())
        .and(with_db(db))
        .and(json_body::<RegisterDTO>());
    unimplemented!()
}

// Return logsessid and salt+pubkey
fn get_login(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("login" / String)
        .and(warp::path::end())
        .and(warp::get())
        .and(with_db(db))
        .and_then(start_login)
}
fn set_login(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("login" / String)
        .and(warp::path::end())
        .and(warp::post())
        .and(with_db(db))
        .and(json_body::<PubKeyDTO>())
        .and_then(handle_client_pubkey)
}
fn verify_login(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("login" / String)
        .and(warp::path::end())
        .and(warp::put())
        .and(with_db(db))
        .and(json_body::<VerifyDTO>())
        .and_then(handle_client_hmac)
}
