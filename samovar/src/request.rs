use std::net::{SocketAddr, TcpStream};

use crate::{common::*, parser::RequestParser};

pub struct Request {
    headers: Vec<Header>,
    body: RequestBody,
    uri: String,
    uri_params: Vec<Params>,
    uri_paths: Vec<String>,
    bare_uri: String,
    method: Method,
    port: u32,
    userinfo: UserInfo,
    host: String,
    scheme: String,
    referer: String,
    content_type: MimeType,
    ip: SocketAddr,
    location: String,
}

impl Request {
    pub fn from(s: &TcpStream) -> Self {
        let req = RequestParser::parse_and_create(s);

        req
    }

    pub fn new(headers: Vec<Header>,
        body: RequestBody,
        uri: String,
        uri_params: Vec<Params>,
        uri_paths: Vec<String>,
        bare_uri: String,
        method: Method,
        port: u32,
        userinfo: UserInfo,
        host: String,
        scheme: String,
        referer: String,
        content_type: MimeType,
        ip: SocketAddr,
        location: String) -> Self {
            Request {
                method,
                uri,
                headers,
                host,
                referer,
                content_type,
                scheme,
                uri_params,
                userinfo,
                bare_uri,
                port,
                uri_paths,
                body,
                ip,
                location,
            }
        }

    pub fn compose_name_for_fs(&self) -> String {
        let name = format!("dir_server_{}", &self.uri.replace("/", "-"));

        name
    }

    pub fn compose_name_for_ep(&self) -> String {
        let ep_name = self.uri.clone();

        let mut ep_name_chars = ep_name.chars().collect::<Vec<char>>();

        if ep_name_chars.len() > 2 {
            if &ep_name_chars[0] == &'/' {
                ep_name_chars.remove(0);
            }

            if &ep_name_chars.last().unwrap() == &&'/' {
                ep_name_chars.pop();
            }

        }   

        let ep_name = String::from_iter(ep_name_chars.iter());

        ep_name
    }

    pub fn get_raw_uri(&self) -> String {
        self.uri.clone()
    }

    pub fn get_header(&self, key: &str) -> Option<String> {
        let mut ret: Option<String> = None;
        
        for h in &self.headers {
            if h.key == key.to_lowercase() {
                ret = Some(h.value.clone());
            }   
        }

        ret
    }

    pub fn get_method(&self) -> Method {
        self.method.clone()    
    }

    pub fn get_host(&self) -> String {
        self.host.clone()    
    }

    pub fn get_ip(&self) -> SocketAddr {
        self.ip.clone()
    }

    pub fn get_scheme(&self) -> String {
        self.scheme.clone()
    }

    pub fn get_user_info(&self) -> UserInfo {
        self.userinfo.clone()
    }

    pub fn get_port(&self) -> u32 {
        self.port.clone()
    }

    pub fn get_bare_url(&self) -> String {
        self.bare_uri.clone()
    }

    pub fn get_body(&self) -> RequestBody {
        self.body.clone()
    }

    pub fn get_referer(&self) -> String {
        self.referer.clone()
    }

    pub fn get_location(&self) -> String {
        self.location.clone()
    }

    pub fn get_params(&self) -> Vec<Params> {
        self.uri_params.clone()
    }

    pub fn get_paths(&self) -> Vec<String> {
        self.uri_paths.clone()
    }

    pub fn get_content_type(&self) -> MimeType {
        self.content_type.clone()
    }


    pub fn get_all_headers(&self) -> Vec<Header> {
        self.headers.clone()
    }
}
