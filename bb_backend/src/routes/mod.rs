use serde::Deserialize;
use std::fmt::Debug;

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
