use crate::context::ContextItem;

pub enum HttpMethod {
    Get,
    Post,
    Option,
    Put,
    Delete,
}

pub struct Endpoint {
    uri: String,
    context: Vec<&'static dyn ContextItem>,
    callable: &'static dyn Fn() -> String,
    method: HttpMethod,
    static_files: String,
    

}