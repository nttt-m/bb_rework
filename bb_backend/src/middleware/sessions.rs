use crate::db::DB;
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};
use std::convert::Infallible;
use std::fmt::{Debug, Formatter, Result};
use warp::Filter;

pub struct Token {
    pub _id: i32,
    pub _token: String,
}

struct InvalidCookie;

impl Debug for InvalidCookie {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Invalid or missing cookie!")
    }
}

impl warp::reject::Reject for InvalidCookie {}

pub fn with_authn(db: DB) -> impl Filter<Extract = (Token,), Error = Infallible> + Clone {
    warp::any().map(|| Token {
        _id: 1,
        _token: String::new(),
    })
}

// pub fn with_authn(db: DB) -> impl Filter<Extract = (Token,), Error = warp::Rejection> + Clone {
//     warp::any().and(warp::filters::cookie::optional("BBAuth")).and_then(move |combined: Option<String>|
//         {
//             let db = db.clone();
//             async move {
//                 let db = db.conn.as_ref().clone();
//                 if let Some(cookie) = combined {
//                     let split = cookie.split(',').collect::<Vec<&str>>();
//                     let id = split[0];
//                     let token = split[1];
//                     if let Ok(res_session) = crate::models::sessions::Entity::find()
//                         .filter(Condition::all().add(crate::models::sessions::Column::UserId.contains(id)).add(crate::models::sessions::Column::Token.contains(token)))
//                         .one(&db).await {
//                             if let Some(session) = res_session {
//                                 let token = Token {
//                                     _id: session.user_id,
//                                     _token: session.token
//                                 };
//                                 Ok(token)
//                             } else {
//                                 Err(warp::reject::custom(InvalidCookie))
//                             }
//                         } else {
//                             Err(warp::reject::custom(InvalidCookie))
//                         }
//                 } else {
//                     Err(warp::reject::custom(InvalidCookie))
//                 }
//             }
//         }
//     )
// }
