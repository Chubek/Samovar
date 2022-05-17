use crate::{
    common::{DummyResponseType, HttpStatus, Method, MimeType, ResponseTextWrapper},
    endpoint::Endpoint,
    request::Request,
    response::Response,
};
use std::{collections::HashMap, net::TcpListener, vec};

pub struct Samovar {
    port: u32,
    address: &'static str,
    endpoints: HashMap<String, Box<Endpoint>>,
    bare_paths: Vec<String>,
}

impl Samovar {
    pub fn new(address: &'static str, port: u32) -> Self {
        let endpoints = HashMap::<String, Box<Endpoint>>::new();
        let bare_paths: Vec<String> = vec![];
        Samovar {
            address,
            port,
            endpoints,
            bare_paths,
        }
    }

    pub fn insert_endpoint(&mut self, endpoint: Box<Endpoint>) {
        let ep_name = endpoint.get_uri();

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

        self.bare_paths.push(ep_name.clone());

        let method_str: String = endpoint.get_method().into();

        let key = format!("{}_{}", ep_name, method_str);

        self.endpoints.insert(key, endpoint);
    }

    fn construct_response_404(&mut self) {
        fn response_empty_handler(_: &Request) -> ResponseTextWrapper {
            let temp_lock = crate::common::TEMP_404.lock().unwrap();
            let temp = temp_lock.get_temp();

            let mut resp = Response::<DummyResponseType>::new_string(
                temp,
                MimeType::TextPlain,
                HttpStatus::Http404NotFound,
            );

            resp.compose()
        }

        let ep = Endpoint::new("404".to_string(), &response_empty_handler, Method::GET);

        self.insert_endpoint(Box::new(ep));
    }

    fn construct_response_405(&mut self) {
        fn response_empty_handler(_: &Request) -> ResponseTextWrapper {
            let temp_lock = crate::common::TEMP_405.lock().unwrap();
            let temp = temp_lock.get_temp();

            let mut resp = Response::<DummyResponseType>::new_string(
                temp,
                MimeType::TextPlain,
                HttpStatus::Http404NotFound,
            );

            resp.compose()
        }

        let ep = Endpoint::new("405".to_string(), &response_empty_handler, Method::GET);

        self.insert_endpoint(Box::new(ep));
    }

    pub fn print_endpoint_uris(&self) {
        for (s, e) in &self.endpoints {
            println!("Endpoint URI: {} -> {}", s, e.get_uri());
        }
    }

    pub fn run(&mut self) {
        self.construct_response_404();
        self.construct_response_405();

        let bind = format!("{}:{}", self.address, self.port);

        println!("Starting server on: {}", &bind);

        let listener = TcpListener::bind(bind).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let request = Request::from(&stream);

                    let uri_name = request.get_raw_uri();
                    let method_name_str: String = request.get_method().into();

                    println!("Request returned: \"{}\"", &uri_name);

                    let mut uri_split = uri_name.split("/").collect::<Vec<&str>>();

                    if uri_split.len() > 1 {
                        if uri_split.last().unwrap().split(".").count() > 1 {
                            uri_split.pop();
                        }
                    }
                    let uri_fin = uri_split.join("/");

                    let mutex_stream = std::sync::Mutex::new(stream);
                    let arc_request = std::sync::Arc::new(request);
                    println!("Looking for paths...");
                    if self.bare_paths.contains(&uri_fin) {
                        let uri_key = format!("{}_{}", uri_fin, method_name_str);
                        println!("Serving...");
                        match self.endpoints.get(&uri_key) {
                            Some(endpoint) => endpoint.serve_response(mutex_stream, arc_request),
                            None => {
                                let ep_405 = self.endpoints.get(&"405_GET".to_string()).unwrap();

                                ep_405.serve_response(mutex_stream, arc_request)
                            }
                        }
                    } else {
                        println!("Request 404'd");
                        let ep_404 = self.endpoints.get(&"404_GET".to_string()).unwrap();

                        ep_404.serve_response(mutex_stream, arc_request)
                    }
                }
                Err(err) => panic!("{}", err),
            }
        }
    }
}
