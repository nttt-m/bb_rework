// TODO: Register
// TODO: Login
// TODO: Generate JWT
// TODO: SRP hashing functions

use crate::crypto::{generate_keys, generate_premaster, hash_keys, hash_multiplier};
use crate::db::DB;
use crate::handlers::authentication::read_one_by_username;
use crate::handlers::employees::find_by_auth;
use crate::reply::{reply, reply_json, reply_wh};
use crate::{CACHE, DH};
use chrono::Utc;
use log::info;
use openssl::bn::BigNum;
use pasetors::claims::Claims;
use pasetors::public;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::borrow::Borrow;
use std::convert::Infallible;
use std::ops::Add;
use std::time::Duration;
use warp::http::StatusCode;
use warp::reply::WithHeader;
use warp::Reply;

pub enum LoginSession {
    Calculate {
        private: BigNum,
        public: BigNum,
        nonce: String,
    },
    Verify {
        secret: BigNum,
        nonce: String,
    },
}

#[derive(Debug, Serialize)]
struct LoginStartReply {
    prime: Vec<u8>,
    generator: u8,
    salt: String,
    pubkey: Vec<u8>,
    nonce: String,
}

#[derive(Debug, Serialize)]
struct LoginVerifyReply {
    nonce: String,
}

pub async fn start_login(username: String, db: DB) -> Result<impl warp::Reply, Infallible> {
    let prime = DH.prime_p().to_vec();
    let generator = DH.generator().to_vec();
    let user_opt = read_one_by_username(username.clone(), db).await;
    let nonce: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    log::info!("{:?}", nonce);
    match user_opt {
        Some(user) => {
            let keys = generate_keys(hash_multiplier(), user.verifier.unwrap());
            CACHE.borrow().remove(&username).await;
            CACHE
                .borrow()
                .insert_with_ttl(
                    username,
                    LoginSession::Calculate {
                        private: keys[0].to_owned().unwrap(),
                        public: keys[1].to_owned().unwrap(),
                        nonce: nonce.clone(),
                    },
                    0,
                    Duration::from_secs(150),
                )
                .await;
            info!("Reply - fine");
            reply_json::<LoginStartReply, Infallible>(
                StatusCode::OK,
                Some(Ok(&LoginStartReply {
                    prime,
                    generator: generator[0],
                    salt: user.salt,
                    pubkey: keys[1].to_vec(),
                    nonce,
                })),
            )
        }
        None => reply_json(StatusCode::FORBIDDEN, None),
    }
}

#[derive(Deserialize, Debug)]
pub struct PubKeyDTO {
    pub pubkey: Vec<u8>,
    pub nonce: String,
}

#[derive(Deserialize, Debug)]
pub struct VerifyDTO {
    pub hmac: [u8; 32],
    pub nonce: String,
}

pub async fn handle_client_pubkey(
    username: String,
    db: DB,
    dto: PubKeyDTO,
) -> Result<impl warp::Reply, Infallible> {
    let c = CACHE.borrow().get(&username);
    let c_pub = BigNum::from_slice(&dto.pubkey).unwrap();
    let nonce_new: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    if let Some(session) = c {
        match session.value() {
            LoginSession::Calculate {
                private,
                public,
                nonce,
            } => {
                if nonce != &dto.nonce {
                    return reply_json(StatusCode::FORBIDDEN, None);
                }
                let hash = hash_keys(&c_pub, &public);
                match read_one_by_username(username.clone(), db).await {
                    Some(user) => {
                        let premaster = generate_premaster(
                            hash,
                            BigNum::from_slice(&user.verifier.unwrap()).unwrap(),
                            c_pub.to_owned().unwrap(),
                            &private,
                        );
                        CACHE.borrow().remove(&username).await;
                        CACHE
                            .borrow()
                            .insert_with_ttl(
                                username,
                                LoginSession::Verify {
                                    secret: premaster,
                                    nonce: nonce_new.clone(),
                                },
                                0,
                                Duration::from_secs(150),
                            )
                            .await;
                        reply_json::<LoginVerifyReply, Infallible>(
                            StatusCode::OK,
                            Some(Ok(&LoginVerifyReply { nonce: nonce_new })),
                        )
                    }
                    None => reply_json(StatusCode::FORBIDDEN, None),
                }
            }
            LoginSession::Verify { .. } => reply_json(StatusCode::FORBIDDEN, None),
        }
    } else {
        reply_json(StatusCode::FORBIDDEN, None)
    }
}

#[derive(Serialize)]
struct FinalDTO {
    id: i32,
    admin: bool,
}

pub async fn handle_client_hmac(
    username: String,
    db: DB,
    dto: VerifyDTO,
) -> Result<WithHeader<warp::reply::Json>, Infallible> {
    let c = CACHE.borrow().get(&username);
    info!("{:?}", &dto);
    if let Some(session) = c {
        match session.value() {
            LoginSession::Verify { secret, nonce } => {
                if nonce != &dto.nonce {
                    return reply_wh(StatusCode::FORBIDDEN, None);
                }
                if dto.hmac
                    == hmac_sha256::HMAC::mac(&username.clone().into_bytes(), &secret.to_vec())
                {
                    // TODO: Create cookie + token
                    let user_opt = read_one_by_username(username.clone(), db.clone()).await;
                    if let Some(user) = user_opt {
                        let emp = find_by_auth(user.auth_id, db.clone()).await.unwrap();
                        let now = Utc::now();
                        let exp = now.add(chrono::Duration::minutes(10));
                        let mut claims = Claims::new().unwrap();
                        claims.issuer("Blorebank");
                        claims.audience("Blorebank EMS");
                        claims.expiration(&exp.to_rfc3339());
                        claims.subject(&emp.id.to_string());
                        claims.add_additional("authz", emp.authz).unwrap();
                        let fin = FinalDTO {
                            id: emp.id,
                            admin: emp.authz != 0,
                        };
                        let pub_token = public::sign(
                            &crate::SK,
                            &crate::PK,
                            &claims,
                            Some(b"footer"),
                            Some(b"implicit assertion"),
                        )
                        .unwrap();
                        let r_in = warp::reply::json(&fin);
                        let rep = warp::reply::with_header(r_in, "Set-Cookie", "auth=");
                        Ok(rep)
                    } else {
                        reply_wh(StatusCode::FORBIDDEN, None)
                    }
                } else {
                    reply_wh(StatusCode::FORBIDDEN, None)
                }
            }
            LoginSession::Calculate { .. } => reply_wh(StatusCode::FORBIDDEN, None),
        }
    } else {
        reply_wh(StatusCode::FORBIDDEN, None)
    }
}

fn cycle_auth_token() {}
fn cycle_refresh_token() {}
