#![forbid(unsafe_code)]

use crate::db::DB;
use crate::logger::init_logger;
use crate::routes::{addresses::addresses, authentication::authentication, telephone_numbers::telephone_numbers, employees::employees};
use anyhow::Result;
use dotenv::dotenv;
use hyper::server::Server;
use listenfd::ListenFd;
use log::{info, trace};
use std::convert::Infallible;
use std::env;
use std::process::exit;
use warp::Filter;

mod db;
mod dtos;
mod logger;
mod models;
mod reply;
mod routes;

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

    let db: DB = DB::new(&env::var("DATABASE_URL")?).await?;
    let routes = addresses(db.clone())
        .or(authentication(db.clone()))
        .or(telephone_numbers(db.clone()))
        .or(employees(db.clone()))
        .with(warp::log("trace"));
    let svc = warp::service(routes);
    let make_svc = hyper::service::make_service_fn(|_: _| {
        // the clone is there because not all warp filters impl Copy
        let svc = svc.clone();
        async move { Ok::<_, Infallible>(svc) }
    });

    let mut listenfd = ListenFd::from_env();
    // if listenfd doesn't take a TcpListener (i.e. we're not running via
    // the command above), we fall back to explicitly binding to a given
    // host:port.
    let server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        Server::from_tcp(l).unwrap()
    } else {
        Server::bind(&([127, 0, 0, 1], 3000).into())
    };

    server.serve(make_svc).await?;

    log_handle.shutdown();
    Ok(())
}
