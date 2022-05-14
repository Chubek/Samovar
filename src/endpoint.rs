use crate::{context::Context, common::ResponseTextWrapper, request::Request};
use crate::common::{self, Method};

pub struct Endpoint {
    uri: String,
    callable: &'static dyn Fn(Request, Vec<Context>) -> ResponseTextWrapper,
    method: Method,
}


impl Endpoint {
    pub fn new(uri: String, 
                callable: &'static dyn Fn(Request, Vec<Context>) -> ResponseTextWrapper,
                method: Method) -> Self 
    {

            Endpoint { uri, callable, method }
    
    }
}