#[macro_use]
extern crate macrovar;
use random_string::generate;
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
use std::rc::Rc; 


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

    let ltem = String::from("fff");

    context_global!(get jun from b unwrap);

    println!("{}", got_jun);

    resp.compose()
}

#[route(method = "GET", path = "/get-ip")]
fn get_ip(r: &Request) -> ResponseTextWrapper {
    let ip = r.get_ip();

    let ip_str = format!("IP: {}", &ip);

    let mut resp = Response::<DummyResponseType>::new_string(
        ip_str,
        samovar::common::MimeType::TextHtml,
        HttpStatus::Http200Ok,
    );

    resp.compose()
}

#[memory_session(name = "a")]
#[physical_session(name = "s", filepath = "mm/cache")]
#[route(method = "POST", path = "/test")]
fn test(r: &Request) -> ResponseTextWrapper {   
 /* 

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

 */

    let charset = "1234567st4t5234t635wujhwuydfhjdhj890";

    let a: Option<String> = sess_s_get("abu");

    let result = match a {
        Some(s) => {
            let mut resp = Response::<DummyResponseType>::new_string(
                s, 
                MimeType::TextPlain, 
                HttpStatus::Http200Ok);
            
            sess_s_insert("abu", generate(6, charset).as_str());

            resp.compose()
        },
        None => {
            let mut resp = Response::<DummyResponseType>::new_string(
                "init".to_string(), 
                MimeType::TextPlain, 
                HttpStatus::Http200Ok);
            
            sess_s_insert("abu", generate(6, charset).as_str());

            resp.compose()
        },
    };

/* 
 *
    println!("saggd");

    context_global!(get jun from b);
    println!("got!");
    let ss = got_jun.unwrap();

    println!("{}", ss);

    let mut resp = Response::<DummyResponseType>::new_string(
        format!("{}", ss),
        MimeType::TextPlain,
        HttpStatus::Http200Ok
    );
*/

    result

}

context_global! { create b type String }


#[static_server(glob = "./static/*", path = "/static", index_file = "none")]
#[samovar(addr = "0.0.0.0", port = "1500")]
fn main() {
    modify_405_template("This method aint it yo!".to_string());

    let lm = String::from("fsdf");

    context_global!(insert lm into b key jun);
    context_global!(get jun from b unwrap);

    println!("{}", got_jun);

    context_global!(free jun);

    serve_forever();
}
