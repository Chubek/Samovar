use std::net::TcpStream;
use std::io::BufWriter;
use crate::common::*;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

pub struct Response<'a, T: ResponseBodyType + Deserialize<'a>> {
    pub headers: Vec<Header>,
    pub server: String,
    pub status: HttpStatus,
    pub content_type: MimeType,
    pub body: ResponseBody<'a, T>,
    pub content_length: usize,
}

impl<'a, T: ResponseBodyType + Deserialize<'a>> Response<'a, T> {
    pub fn new(headers: Vec<Header>, 
        server: String, 
        status: HttpStatus, 
        content_type: MimeType, 
        body: ResponseBody<'a, T>,
        content_length: usize) -> Self {
            Response { headers, server, status, content_type, body, content_length }
        }
    pub fn new_json(object: T, status: HttpStatus) -> Self {
        let body = ResponseBody::new(
            MimeType::ApplicationJson, 
            object
        );
        let headers = Vec::<Header>::new();
        let server = String::from("ChubyHttp/0.0.1b");
        let content_length = object.get_length();
        let content_type = MimeType::ApplicationJson;

        Response { headers, server, status, content_type, body, content_length }
    }
}