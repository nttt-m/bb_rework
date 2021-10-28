use crate::{
    db::DB,
    dtos::core::{RegisterDTO, LoginDTO},
    middleware::data::{json_body, with_db},
};
use warp::Filter;

pub fn core(db: DB) {
    // register(db.clone())
    //     .and(login(db.clone()));
    unimplemented!()
}

fn register(db: DB) {
    warp::path!("register")
        .and(warp::path::end())
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(json_body::<RegisterDTO>());
    unimplemented!()
}

// Return logsessid and salt+pubkey
fn get_login(db: DB) {
    warp::path!("login" / String)
        .and(warp::path::end())
        .and(warp::get())
        .and(with_db(db));
        // .and_then();
        unimplemented!();
}
fn set_login(db: DB) {
    warp::path!("login" / String)
        .and(warp::path::end())
        .and(warp::post())
        .and(with_db(db))
        .and(json_body::<LoginDTO>());
        // .and_then();
        unimplemented!();
}
fn verify_login(db: DB) {}
