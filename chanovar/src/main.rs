#[macro_use]
extern crate macrovar;

use std::result;

use samovar::{
    common::{
        modify_405_template, DummyResponseType, HttpStatus, MimeType, RequestBodyType,
        ResponseTextWrapper,
    },
    request::{self, Request},
    response::Response,
};
use serde::{Deserialize, Serialize, __private::de::Content};

#[derive(ResponseCommon, Deserialize, Serialize, Clone)]
struct ResType {
    field_str: String,
    field_header: String,
    field_u32: u32,
}

#[route(method = "GET", path = "/")]
fn index(_: &Request) -> ResponseTextWrapper {
    let mut resp = Response::<DummyResponseType>::new_string(
        "Ok!".to_string(),
        samovar::common::MimeType::TextHtml,
        HttpStatus::Http200Ok,
    );

    resp.compose()
}

#[route(method = "POST", path = "/test")]
fn test(r: &Request) -> ResponseTextWrapper {
    let field_header = r.get_header("test-header").unwrap();
    let field_str = String::from("Served by Samovar");
    let field_u32 = 12u32;

    let res = ResType {
        field_header,
        field_str,
        field_u32,
    };

    let status = HttpStatus::Http202Accepted;

    let req_body = r.get_body();

    let contents = req_body.content;

    let result = match contents {
        RequestBodyType::Json(j) => {
            let mut resp = Response::<DummyResponseType>::new_string(
                j["aa"].to_string(),
                MimeType::TextPlain,
                status,
            );

            resp.compose()
        }
        RequestBodyType::Str(t) => {
            let mut resp =
                Response::<DummyResponseType>::new_string(t, MimeType::TextPlain, status);

            resp.compose()
        }
    };

    result
}

#[static_server(glob = "./static/*", path = "/static", index_file = "none")]
#[samovar(addr = "0.0.0.0", port = "1500")]
fn main() {
    modify_405_template("This method aint it yo!".to_string());

    serve_forever();
}
