use anyhow::Result;
use serde::Serialize;
use std::fmt::Display;
use std::{convert::Infallible, fmt::Debug};
use warp::reply::{with_header, WithHeader};
use warp::{http::StatusCode, reply::json, Reply};

pub fn reply<T, U>(
    status: StatusCode,
    message: Option<Result<&T, &U>>,
) -> Result<impl Reply, Infallible>
where
    T: Debug,
    U: Debug,
{
    if let Some(r) = message {
        let reply;
        match r {
            Ok(m) => {
                reply = format!(
                    "{{\"status\": \"{}\", \"message\": \"{:?}\"}}",
                    status.as_str(),
                    m
                );
            }
            Err(m) => {
                reply = format!(
                    "{{\"status\": \"{}\", \"message\": \"{:?}\"}}",
                    status.as_str(),
                    m
                );
            }
        }
        Ok(json(&reply))
    } else {
        let reply = format!("{{\"status\": \"{}\"}}", status.as_str());
        Ok(json(&reply))
    }
}

#[derive(Serialize)]
struct ReplyStruct<X>
where
    X: Serialize,
{
    status: u16,
    message: X,
}

pub fn reply_json<T, U>(
    status: StatusCode,
    message: Option<Result<&T, &U>>,
) -> Result<impl Reply, Infallible>
where
    T: Serialize,
    U: Debug,
{
    if let Some(r) = message {
        match r {
            Ok(m) => Ok(json(&ReplyStruct {
                status: status.as_u16(),
                message: m,
            })),
            Err(m) => Ok(json(&ReplyStruct {
                status: status.as_u16(),
                message: &format!("{:?}", m),
            })),
        }
    } else {
        let reply = format!("{{status: {}}}", status.as_str());
        Ok(json(&reply))
    }
}

pub fn reply_wh(
    status: StatusCode,
    message: Option<Result<bool, Infallible>>,
) -> Result<WithHeader<warp::reply::Json>, Infallible> {
    if let Some(r) = message {
        let reply;
        match r {
            Ok(m) => {
                reply = format!(
                    "{{\"status\": {}, \"message\": \"{:?}\"}}",
                    status.as_str(),
                    m
                );
            }
            Err(m) => {
                reply = format!(
                    "{{\"status\": {}, \"message\": \"{:?}\"}}",
                    status.as_str(),
                    m
                );
            }
        }
        Ok(with_header(json(&reply), "", ""))
    } else {
        let reply = format!("{{\"status\": {}}}", status.as_str());
        Ok(with_header(json(&reply), "", ""))
    }
}
