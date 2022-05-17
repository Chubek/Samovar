#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

pub mod common;
pub mod endpoint;
pub mod parser;
pub mod request;
pub mod response;
pub mod samovar;
pub mod staticserver;
pub mod session;

#[cfg(test)]
mod tests {}
