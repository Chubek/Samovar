use crate::common::Method;
use crate::{common::ResponseTextWrapper, request::Request};
use std::net::TcpStream;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Endpoint {
    uri: String,
    callable: &'static (dyn Fn(&Request) -> ResponseTextWrapper + Sync),
    method: Method,
}

impl Endpoint {
    pub fn new_string_method(
        uri: String,
        callable: &'static (dyn Fn(&Request) -> ResponseTextWrapper + Sync),
        method_string: String,
    ) -> Self {
        println!("Serving endpoint at {} with method {}", &uri, &method_string.to_uppercase());

        let method: Method = method_string.into();

        Endpoint {
            uri,
            callable,
            method,
        }
    }

    pub fn new(
        uri: String,
        callable: &'static (dyn Fn(&Request) -> ResponseTextWrapper + Sync),
        method: Method,
    ) -> Self {
        let method_str: String = method.clone().into();

        println!("Serving endpoint at {} with method {}", &uri, method_str);


        Endpoint {
            uri,
            callable,
            method,
        }
    }

    pub fn serve_response(
        &self,
        stream: Mutex<TcpStream>,
        request: Arc<Request>,
    ) {
        let callable_arc = Arc::new(self.callable);

        thread::spawn(move || {
            let mut stream_deref = stream.lock().unwrap();
            let request_deref = request.deref();
            let callabale_deref = *callable_arc.deref();

            let resp = (callabale_deref)(request_deref);

            resp.serve(&mut stream_deref);
        });
    }

    pub fn get_uri(&self) -> String {
        self.uri.clone()
    }

    pub fn get_method(&self) -> Method {
        self.method.clone()
    }
}
