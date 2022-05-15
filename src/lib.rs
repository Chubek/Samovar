#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

mod common;
mod context;
mod endpoint;
mod parser;
mod request;
mod response;
mod samovar;
mod staticserver;

#[cfg(test)]
mod tests {}
