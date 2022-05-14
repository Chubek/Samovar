use std::net::TcpStream;
use std::ops::Deref;
use std::sync::Arc;
use std::thread;

use crate::{context::Context, common::ResponseTextWrapper, request::Request};
use crate::common::Method;

pub struct Endpoint {
    uri: String,
    callable: &'static (dyn Fn(&Request, &Vec<Context>) -> ResponseTextWrapper + Sync),
    method: Method,
}


impl Endpoint {
    pub fn new(uri: String, 
                callable: &'static (dyn Fn(&Request, &Vec<Context>) -> ResponseTextWrapper + Sync),
                method: Method) -> Self 
    {

            Endpoint { uri, callable, method }
    
    }


    pub fn serve_response(&self, stream: Arc<TcpStream>, context: Arc<Vec<Context>>, request: Arc<Request>) {
        let callable_arc = Arc::new(self.callable);
        
        thread::spawn(move || {
            let stream_deref = stream.deref();
            let context_deref = context.deref();
            let request_deref = request.deref();
            let callabale_deref = *callable_arc.deref();

            let resp = (callabale_deref)(request_deref, context_deref);

            resp.serve(stream_deref);

        });
    }
}