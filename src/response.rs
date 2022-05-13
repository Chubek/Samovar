use crate::common::*;
use chrono::{DateTime, Utc, Date};
use serde::{Deserialize, Serialize};
use std::io::BufWriter;
use std::net::TcpStream;

pub struct Response<'a, T: Clone + ResponseCommon + Deserialize<'a>> {
    pub headers: Vec<Header>,
    pub server: String,
    pub status: HttpStatus,
    pub content_type: MimeType,
    pub body: ResponseBody<'a, T>,
    pub content_length: usize,
    pub datetime: DateTime<Utc>,
}

impl<'a, T: Clone + ResponseCommon + Deserialize<'a>> Response<'a, T> {
    pub fn new(
        headers: Vec<Header>,
        server: String,
        status: HttpStatus,
        content_type: MimeType,
        body: ResponseBody<'a, T>,
        content_length: usize,
    ) -> Self {
        Response {
            headers,
            server,
            status,
            content_type,
            body,
            content_length,
            datetime: Utc::now(),
        }
    }
    pub fn new_json(object: T, status: HttpStatus) -> Self {
        let body = ResponseBody::new_json(object.clone());
        let headers = Vec::<Header>::new();
        let server = String::from("ChubyHttp/0.0.1b");
        let content_length = object.get_length();
        let content_type = MimeType::ApplicationJson;
        let datetime = Utc::now();

        Response {
            headers,
            server,
            status,
            content_type,
            body,
            content_length,
            datetime,
        }
    }

    pub fn new_string(t: String, content_type: MimeType, status: HttpStatus) -> Self {
        let body = ResponseBody::new_string(content_type.clone(), t.clone());
        let headers = Vec::<Header>::new();
        let server = String::from("ChubyHttp/0.0.1b");
        let content_length = t.len();
        let datetime = Utc::now();

        Response {
            headers,
            server,
            status,
            content_type,
            body,
            content_length,
            datetime,
        }
    }
}
