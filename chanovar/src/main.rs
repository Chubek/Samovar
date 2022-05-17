#[macro_use]
extern crate macrovar;

use samovar::{request::Request, common::{ResponseTextWrapper, DummyResponseType, HttpStatus}, response::Response};
use serde::{Serialize, Deserialize};

#[derive(ResponseCommon, Deserialize, Serialize, Clone)]
struct ResType {
    field_str: String,
    field_header: String,
    field_u32: u32,
}


#[route(method = "GET", path = "/", index = 0, group = "main")]
fn index(_: &Request) -> ResponseTextWrapper {
    let mut resp = Response::<DummyResponseType>::new_string(
              "Ok!".to_string(), 
     samovar::common::MimeType::TextHtml, 
          HttpStatus::Http200Ok);

    resp.compose()
}

#[route(method = "GET", path = "/test", index = 2, group = "main")]
fn test(r: &Request) -> ResponseTextWrapper {
    let field_header = r.get_header("test-header").unwrap();
    let field_str = String::from("Served by Samovar");
    let field_u32 = 12u32;

    let res = ResType {field_header, field_str, field_u32};

    let status = HttpStatus::Http202Accepted;

    let mut resp = Response::<ResType>::new_json(res, status);

    resp.compose()
}

#[static_server(glob = "./static/*", path = "/static", group = "main", index_file = "None", index = 1)]
#[samovar(addr = "0.0.0.0", port = "8545", groups = "(main; 0..2)")]
fn main() {

    serve_forever();

}
