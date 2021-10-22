use sea_orm::ColumnTrait;
use serde::{de::DeserializeOwned, Deserialize};
use std::convert::Infallible;
use warp::body::{content_length_limit, json};
use warp::Filter;
use crate::db::DB;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::Condition;
use std::fmt::Debug;
use std::fmt;

pub mod addresses;
pub mod authentication;
pub mod core;
pub mod employees;
pub mod telephone_numbers;

#[derive(Debug, Deserialize)]
pub struct ListOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

pub struct Token {
    id: i32,
    token: String
}

struct InvalidCookie;

impl Debug for InvalidCookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid or missing cookie!")
    }
}

impl warp::reject::Reject for InvalidCookie {}

pub fn with_authn(db: DB) -> impl Filter<Extract = (Token,), Error = warp::Rejection> + Clone {
    warp::any().and(warp::filters::cookie::optional("BBAuth")).and_then(move |combined: Option<String>| 
        {
            let db = db.clone(); 
            async move {
                let db = db.conn.as_ref().clone();
                if let Some(cookie) = combined {
                    let split = cookie.split(',').collect::<Vec<&str>>();
                    let id = split[0];
                    let token = split[1];
                    if let Ok(res_session) = crate::models::sessions::Entity::find()
                        .filter(Condition::all().add(crate::models::sessions::Column::UserId.contains(id)).add(crate::models::sessions::Column::Token.contains(token)))
                        .one(&db).await {
                            if let Some(session) = res_session {
                                let token = Token {
                                    id: session.user_id,
                                    token: session.token
                                };
                                Ok(token)
                            } else {
                                Err(warp::reject::custom(InvalidCookie))
                            }
                        } else {
                            Err(warp::reject::custom(InvalidCookie))
                        }
                } else {
                    Err(warp::reject::custom(InvalidCookie))
                }
            }
        }
    )
}

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
