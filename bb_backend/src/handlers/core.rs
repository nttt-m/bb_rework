// pub async fn get_by_uname(uname: String, db: DB) -> Option<[String; 3]> {
//     let auth = crate::models::authentication::Entity::find().filter(crate::models::authentication::Column::Username.contains(&uname)).one(db.conn.as_ref()).await;
//     match auth {
//         Ok(a) => {
//             match &a {
//                 Some(b) => {
//                     Some([b.auth_id.to_string(), b.username.clone(), b.password.clone()])
//                 },
//                 None => {
//                     None
//                 }
//             }
//         }
//         Err(e) => {
//             None
//         }
//     }
// }

// pub async fn get_id_by_auth_id(id: i32, db: DB) -> Option<i32> {
//     let auth = crate::models::employees::Entity::find().filter(crate::models::employees::Column::Auth.contains(&id.to_string())).one(db.conn.as_ref()).await;
//     match auth {
//         Ok(a) => {
//             match &a {
//                 Some(b) => {
//                     Some(b.id)
//                 },
//                 None => {
//                     None
//                 }
//             }
//         }
//         Err(e) => {
//             None
//         }
//     }
// }

// pub async fn create_session(id: i32, db: DB) -> anyhow::Result<String> {
//     let token = uuid::Uuid::new_v4().to_string();
//     let sess = crate::models::sessions::ActiveModel {
//         user_id: Set(id),
//         token: Set(token.clone()),
//         ..Default::default()
//     };
//     match sess.insert(db.conn.as_ref()).await {
//         Ok(r) => {
//             Ok(token)
//         },
//         Err(e) => {
//             Err(e.into())
//         }
//     }
// }

// TODO: Register
// TODO: Login
// TODO: Generate JWT
// TODO: SRP hashing functions

use crate::db::DB;

pub fn start_login(id: i32, db: DB) {

}