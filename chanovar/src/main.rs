#[macro_use]
extern crate macrovar;

use samovar::{samovar::Samovar, request::Request, context::ContextType, common::ResponseTextWrapper};


#[route(method = "GET", path = "/", index = 0, group = "main")]
fn index(_: &Request, _: &ContextType) -> ResponseTextWrapper {
    ResponseTextWrapper::new("a".to_string())
}
#[samovar(addr = "localhost", port = "92", groups = "(main; 0)")]
fn main() {
    println!("hello world")

}
