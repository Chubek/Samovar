use serde::__private::de::IdentifierDeserializer;
use serde::{Serialize, Deserialize};

use crate::common::{self, *};
use crate::request::Request;
use std::net::TcpStream;
use std::io::{BufReader, Read};

pub struct RequestParser;

impl RequestParser {
    pub fn parse_and_create(stream: &TcpStream) -> Request {
        let req = Self::read_data(stream);

        let method = Self::get_method(&req);
        let location = Self::get_location(&req);
        let uri = Self::get_uri(&location);
        let headers = Self::get_headers(&req);
        let host = Self::get_host(&headers);
        let referer = Self::get_referer(&headers);
        let content_type = Self::get_ctype(&headers);
        let scheme = Self::get_scheme(&location);
        let uri_params = Self::get_url_params(&location);
        let userinfo = Self::get_userpass(&location);
        let bare_uri = Self::get_bare_url(&location);
        let port = Self::get_port(&location);
        let uri_paths = Self::get_uri_paths(&location);
        let body = Self::get_body(&req, content_type.clone());
        let ip = stream.local_addr().unwrap();


        Request {method, 
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

    fn read_data(stream: &TcpStream) -> String {
        let mut reader = BufReader::new(stream);

        let mut data = Vec::<u8>::new();

        reader.read_to_end(&mut data).unwrap();

        let ret = String::from_utf8(data).unwrap();

        ret
    }

    fn get_method(req: &String) -> Method {
        let first_line = req.lines().into_iter().collect::<Vec<&str>>()[0];

        let first_word = first_line
            .split_whitespace()
            .into_iter()
            .collect::<Vec<&str>>()[0]
            .to_string();

        let method_from: Method = first_word.into();

        method_from
        
    }

    fn get_location(req: &String) -> String {
        let first_line = req.lines().into_iter().collect::<Vec<&str>>()[0];

        match first_line
            .split_whitespace()
            .into_iter()
            .collect::<Vec<&str>>()
            .get(1)
        {
            Some(uri) => uri.to_string(),
            None => panic!("Wrong request"),
        }
    }

    fn get_headers(req: &String) -> Vec<Header> {
        let length = req.clone().lines().count();

        let ret = req
            .lines()
            .into_iter()
            .enumerate()
            .filter(|&(i, l)| i >= 2 && l.split(": ").count() == 2)
            .map(|(_, l)| {
                let l_split = l.split(": ").collect::<Vec<&str>>();

                Header {
                    key: l_split[0].to_string(),
                    value: l_split[1].to_string(),
                }
            })
            .collect();

        ret
    }

    fn get_host(headers: &Vec<Header>) -> String {
        let mut ret = String::new();

        for h in headers.into_iter() {
            if h.key == "host" {
                ret = h.value.clone();
            }
        }

        ret
    }

    fn get_referer(headers: &Vec<Header>) -> String {
        let mut ret = String::new();

        for h in headers.into_iter() {
            if h.key == "referer" {
                ret = h.value.clone();
            }
        }

        ret
    }

    fn get_ctype(headers: &Vec<Header>) -> MimeType {
        let mut ret = String::new();

        for h in headers.into_iter() {
            if h.key == "content-type" {
                ret = h.value.clone();
            }
        }

        match ret.as_str() {
            "application/json" => MimeType::ApplicationJson,
            "text/plain" => MimeType::TextPlain,
            "text/html" => MimeType::TextHtml,
            "application/octet-stream" => MimeType::ApplicationOctetStream,
            _ => MimeType::TextPlain,
        }
    }

    fn get_scheme(location: &String) -> String {
        let uri_split = location.split("://").collect::<Vec<&str>>();

        let mut scheme = String::from("http");

        if uri_split.len() == 2 {
            scheme = uri_split[0].to_string();
        }

        scheme
    }

    fn get_userpass(location: &String) -> UserInfo {
        let uri_split = location.split("@").collect::<Vec<&str>>();

        let mut username = String::new();
        let mut password = String::new();

        if uri_split.len() >= 2 {
            let mut user_pass_str = "";
            if uri_split[0].split("://").count() == 2 {
                user_pass_str = uri_split[0].split("://").collect::<Vec<&str>>()[1];
            } else {
                user_pass_str = uri_split[0];
            }

            let split_on_colon = user_pass_str.split(":").collect::<Vec<&str>>();

            username = split_on_colon[0].to_string();
            password = split_on_colon[1].to_string();
        }

        UserInfo { username, password }
    }

    fn get_port(location: &String) -> u32 {
        let last_str = location.split(":").into_iter().last();

        match last_str {
            Some(last) => match last.parse::<u32>() {
                Ok(res) => res,
                Err(_) => 80u32,
            },
            None => 80u32,
        }
    }

    fn get_url_params(location: &String) -> Vec<Params> {
        let uri_split = location.split("?q=").collect::<Vec<&str>>();

        let mut ret: Vec<Params> = vec![];

        if uri_split.len() == 2 {
            let last = uri_split.into_iter().last().unwrap();

            ret = last
                .split("&")
                .into_iter()
                .map(|x| {
                    let x_split = x.split("=").collect::<Vec<&str>>();

                    Params {
                        key: x_split[0].to_string(),
                        value: x_split[1].to_string(),
                    }
                })
                .collect()
        }

        ret
    }

    fn get_bare_url(location: &String) -> String {
        let uri_split = location.split("?q=").into_iter().collect::<Vec<&str>>();

        uri_split[0].to_string()
    }

    fn get_uri_paths(uri: &String) -> Vec<String> {
        let uri_split = uri.split("/");

        let mut ret: Vec<String> = vec![];

        if uri_split.clone().count() > 1 {
            ret = uri_split
                .into_iter()
                .enumerate()
                .filter(|&(i, _)| i > 1 && i < uri.split("/").count())
                .map(|(_, x)| x.to_string())
                .collect();
        }

        ret
    }

    fn get_uri(location: &String) -> String {
        let mut uri_split = location.split("/").collect::<Vec<&str>>();

        uri_split.remove(0);

        let uri = uri_split.join("/");

        let uri_no_params = uri.split("?q=").next().unwrap().to_string();

        uri_no_params
    }

    fn get_body(req: &String, ctype: MimeType) -> RequestBody {
        let req_split = req.split("\n\n");

        let mut ret = String::new();

        if req_split.count() == 2 {
            if let Some(b) = req.split("\n\n").into_iter().last() {
                ret = b.to_string();
            }
        }

        RequestBody::from_str(ret, ctype)
    }
}


