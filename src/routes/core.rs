use crate::{
    db::DB,
    middleware::data::{json_body, with_db},
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Deserialize, Serialize)]
struct LoginDTO {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
struct RegisterDTO {
    pub first_name: String,
    pub last_name: String,
    pub position: String,
    pub house_name_number: String,
    pub street: Option<String>,
    pub town_city: Option<String>,
    pub region: Option<String>,
    pub postal_code: String,
    pub country: String,
    pub username: String,
    pub password: String,
    pub telephone_number: String,
}

fn register() {}

fn login(db: DB) {
    warp::path!("login")
        .and(warp::path::end())
        .and(warp::post())
        .and(with_db(db))
        .and(json_body::<LoginDTO>());
        // .and_then();
        unimplemented!();
}

pub async fn get_by_uname(uname: String, db: DB) -> Option<[String; 3]> {
    let auth = crate::models::authentication::Entity::find().filter(crate::models::authentication::Column::Username.contains(&uname)).one(db.conn.as_ref()).await;
    match auth {
        Ok(a) => {
            match &a {
                Some(b) => {
                    Some([b.auth_id.to_string(), b.username.clone(), b.password.clone()])
                },
                None => {
                    None
                }
            }
        }
        Err(e) => {
            None
        }
    }
}

pub async fn get_id_by_auth_id(id: i32, db: DB) -> Option<i32> {
    let auth = crate::models::employees::Entity::find().filter(crate::models::employees::Column::Auth.contains(&id.to_string())).one(db.conn.as_ref()).await;
    match auth {
        Ok(a) => {
            match &a {
                Some(b) => {
                    Some(b.id)
                },
                None => {
                    None
                }
            }
        }
        Err(e) => {
            None
        }
    }
}

pub async fn create_session(id: i32, db: DB) -> anyhow::Result<String> {
    let token = uuid::Uuid::new_v4().to_string();
    let sess = crate::models::sessions::ActiveModel {
        user_id: Set(id),
        token: Set(token.clone()),
        ..Default::default()
    };
    match sess.insert(db.conn.as_ref()).await {
        Ok(r) => {
            Ok(token)
        },
        Err(e) => {
            Err(e.into())
        }
    }
}
