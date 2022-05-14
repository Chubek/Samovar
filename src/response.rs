use crate::common::*;
use chrono::{Date, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io::BufWriter;
use std::net::TcpStream;

pub struct Response<'a, T: Clone + ResponseCommon + Deserialize<'a>> {
    headers: Vec<Header>,
    server: String,
    status: HttpStatus,
    content_type: MimeType,
    body: ResponseBody<'a, T>,
    content_length: usize,
    datetime: DateTime<Utc>,
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
        let server = String::from("ChubyHttp/0.0.1b");
        let content_length = object.get_length();
        let content_type = MimeType::ApplicationJson;
        let datetime = Utc::now();
        let headers = vec![];

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
        let server = String::from("ChubyHttp/0.0.1b");
        let content_length = t.len();
        let datetime = Utc::now();
        let headers = vec![];

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


    fn format_date_add_header(&mut self) {
        let str_date = self.datetime.format("%a, %d %B %Y %T %Z").to_string();
        let header_date = Header {
            key: "Date".to_string(),
            value: str_date,
        };

        self.headers.push(header_date)
    }

    fn format_content_add_header(&mut self) {
        let header_ctype = Header {
            key: "Content-Type".to_string(),
            value: self.content_type.clone().into(),
        };
        let header_clength = Header {
            key: "Content-Length".to_string(),
            value: self.content_length.to_string(),
        };

        self.headers.push(header_ctype);
        self.headers.push(header_clength);
    }

    fn format_response_metadata(&self) -> String {
        let status_string: String = self.status.into();

        let ret = format!("HTTP/1.1 {}\nServer: {}", status_string, self.server);

        ret
    }

    fn sort_header_vec(&mut self) {
        self.headers.sort_by_key(|x| x.key.clone());
    }

    pub fn set_header(&mut self, key: String, value: String) {
        if self.headers.iter().filter(|x| x.key == key).count() > 0 {
            for h in self.headers.iter_mut() {
                if h.key == key {
                    h.value = value.clone()
                }
            }
        } else {
            let new_header = Header {
                key: key.clone(),
                value: value.clone(),
            };

            self.headers.push(new_header);
        }
    }

    fn make_body(&self) -> String {
        match self.body.content.clone() {
            ResponseBodyType::Object(obj) => obj.parse_to_string(),
            ResponseBodyType::Str(t) => t,
            ResponseBodyType::PhantomData(_) => "".to_string(),
        }
    }

    fn make_header_single(h: &Header) -> String {
        let ret = format!("{}: {}", h.key, h.value);

        ret
    }

    fn make_header(&mut self) -> String {
        self.format_date_add_header();
        self.format_content_add_header();
        self.sort_header_vec();
        

        let h_joined = self
            .headers
            .iter()
            .map(|x| Self::make_header_single(x))
            .collect::<Vec<String>>()
            .join("\n");
        h_joined
    }

    pub fn compose(&mut self) -> ResponseTextWrapper {
        let metadata = self.format_response_metadata();
        let headers_joined = self.make_header();
        let body = self.make_body();

        let ret = format!("{}\n{}\n\n{}", metadata, headers_joined, body);

        ResponseTextWrapper::new(ret)
    }
}
