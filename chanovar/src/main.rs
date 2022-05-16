#[macro_use]
extern crate macrovar;

use samovar::{request::Request, context::ContextType, common::ResponseTextWrapper};
use samovar::staticserver::StaticServerType;

#[route(method = "GET", path = "/", index = 0, group = "main")]
fn index(_: &Request, _: &ContextType) -> ResponseTextWrapper {
    ResponseTextWrapper::new("a".to_string())
}
#[samovar(addr = "0.0.0.0", port = "8545", groups = "(main; 0)")]
fn main() {
    let fs = StaticServerType::ServeWithoutIndex("static/*".to_string());

    add_new_static_file_server(fs);

    serve_forever();

}
