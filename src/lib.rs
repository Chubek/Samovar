#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;


mod transport;
mod request;
mod common;
mod parser;
mod response;
mod staticserver;
mod context;
mod endpoint;

#[cfg(test)]
mod tests {    


}
