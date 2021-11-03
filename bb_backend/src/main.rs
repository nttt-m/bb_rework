#![forbid(unsafe_code)]

use crate::crypto::load_dh;
use crate::db::DB;
use crate::handlers::core::LoginSession;
use crate::logger::init_logger;
use crate::routes::core::core;
use crate::routes::{addresses::addresses, authentication::authentication, employees::employees};
use anyhow::Result;
use dotenv::dotenv;
use ed25519_dalek::Keypair;
use hyper::server::Server;
use lazy_static::lazy_static;
use listenfd::ListenFd;
use log::{info, trace};
use openssl::{dh::Dh, pkey::Params};
use pasetors::keys::{AsymmetricPublicKey, AsymmetricSecretKey, V4};
use rand_core::OsRng;
use std::convert::Infallible;
use std::env;
use std::process::exit;
use stretto::{AsyncCache, DefaultKeyBuilder};
use warp::http::Method;
use warp::Filter;

mod crypto;
mod db;
mod dtos;
mod handlers;
mod logger;
mod middleware;
mod models;
mod reply;
mod routes;

lazy_static! {
    pub static ref CACHE: AsyncCache<String, LoginSession, DefaultKeyBuilder> =
        AsyncCache::new(12960, 1e6 as i64, DefaultKeyBuilder::default()).unwrap();
    pub static ref DH: Dh<Params> = load_dh();
    pub static ref keypair: Keypair = gen_keypair();
    pub static ref SK: AsymmetricSecretKey<V4> =
        AsymmetricSecretKey::<V4>::from(&keypair.secret.to_bytes()).unwrap();
    pub static ref PK: AsymmetricPublicKey<V4> =
        AsymmetricPublicKey::<V4>::from(&keypair.public.to_bytes()).unwrap();
}

fn gen_keypair() -> Keypair {
    let mut csprng: OsRng = OsRng {};
    Keypair::generate(&mut csprng)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    let log_handle = match init_logger() {
        Ok(l) => {
            trace!("Logger online!");
            l
        }
        Err(e) => {
            eprintln!("Unable to initialise logger: {:?}", e);
            exit(1)
        }
    };
    info!("Starting BB-HRMS v0.1.0");

    let cors = warp::cors()
        .allow_origins(["http://127.0.0.1:8800", "http://localhost:8800"])
        .allow_methods(&[
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_header("content-type")
        .allow_credentials(true)
        .build();

    let db: DB = DB::new(&env::var("DATABASE_URL")?).await?;
    let routes = addresses(db.clone())
        .or(authentication(db.clone()))
        .or(core(db.clone()))
        .or(employees(db.clone()))
        .recover(handle_rejection)
        .with(cors.clone())
        .with(warp::log("info"));
    let routes2 = core(db.clone())
        .recover(handle_rejection)
        // .with(cors)
        .with(warp::log("info"));
    let svc = warp::service(routes2);
    let make_svc = hyper::service::make_service_fn(|_: _| {
        // the clone is there because not all warp filters impl Copy
        let svc = svc.clone();
        async move { Ok::<_, Infallible>(svc) }
    });

    let mut listenfd = ListenFd::from_env();
    // if listenfd doesn't take a TcpListener (i.e. we're not running via
    // the command above), we fall back to explicitly binding to a given
    // host:port.
    // systemfd --no-pid -s http::3000 -- cargo watch -x 'run'
    let server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        Server::from_tcp(l).unwrap()
    } else {
        Server::bind(&([127, 0, 0, 1], 3000).into())
    };

    server.serve(make_svc).await?;

    log_handle.shutdown();
    Ok(())
}

async fn handle_rejection(
    err: warp::Rejection,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(warp::reply::json(&format!("{:?}", err)))
}
