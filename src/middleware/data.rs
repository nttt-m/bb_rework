use crate::db::DB;
use serde::de::DeserializeOwned;
use std::convert::Infallible;
use warp::{Filter, body::{content_length_limit, json}};

pub fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn json_body<E>() -> impl Filter<Extract = (E,), Error = warp::Rejection> + Clone
where
    E: DeserializeOwned + Send,
{
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    content_length_limit(1024 * 16).and(json())
}