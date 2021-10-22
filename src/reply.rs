use anyhow::Result;
use std::{convert::Infallible, fmt::Debug};
use warp::{http::StatusCode, reply::json, Reply};

pub fn reply<T, U>(status: StatusCode, message: Option<Result<&T, &U>>) -> Result<impl Reply, Infallible>
where
    T: Debug,
    U: Debug
{
    if let Some(r) = message {
        let reply;
        match r {
            Ok(m) => {
                reply = format!("{{status: {}, message: {:?}}}", status.as_str(), m);
            }
            Err(m) => {
                reply = format!("{{status: {}, message: {:?}}}", status.as_str(), m);
            }
        }
        Ok(json(&reply))
    } else {
        let reply = format!("{{status: {}}}", status.as_str());
        Ok(json(&reply))
    }
}
    