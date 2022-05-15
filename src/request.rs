use std::net::{SocketAddr, TcpStream};

use crate::{common::*, parser::RequestParser};

pub struct Request {
    pub headers: Vec<Header>,
    pub body: RequestBody,
    pub uri: String,
    pub uri_params: Vec<Params>,
    pub uri_paths: Vec<String>,
    pub bare_uri: String,
    pub method: Method,
    pub port: u32,
    pub userinfo: UserInfo,
    pub host: String,
    pub scheme: String,
    pub referer: String,
    pub content_type: MimeType,
    pub ip: SocketAddr,
    pub location: String,
}


impl Request {
    pub fn from(s: &TcpStream) -> Self {
        let req = RequestParser::parse_and_create(s);

        req
    }

    pub fn compose_name_for_fs(&self) -> String {
        let name = format!("request_server_{}", self.uri.replace("/", "-"));

        name
    }

}