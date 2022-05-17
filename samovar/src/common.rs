use core::panic;
use std::{marker::PhantomData, net::{TcpStream, Shutdown}, io::Write, sync::Mutex};
use serde::Deserialize;
use serde_json::{from_str, Value};

#[derive(Clone)]
pub struct Header {
    pub key: String,
    pub value: String,
}
#[derive(Clone, Hash, Eq, PartialEq)]
pub enum MimeType {
    ApplicationJson,
    TextPlain,
    TextHtml,
    ApplicationOctetStream,
    TextJavaScript,
    TextCSS,
}

impl Into<String> for MimeType {
    fn into(self) -> String {
        match self {
            MimeType::ApplicationJson => "application/json".to_string(),
            MimeType::TextPlain => "text/plain".to_string(),
            MimeType::TextHtml => "text/html".to_string(),
            MimeType::ApplicationOctetStream => "application/octet-stream".to_string(),
            MimeType::TextJavaScript => "text/javascript".to_string(),
            MimeType::TextCSS => "text/css".to_string(),
        }
    } 
}

#[derive(Clone)]
pub enum RequestBodyType {
    Json(Value),
    Str(String),
}

#[derive(Clone)]
pub struct RequestBody {
    pub content_type: MimeType,
    pub content: RequestBodyType,
}

impl RequestBody {
    pub fn from_str(t: String, ctype: self::MimeType) -> Self {
        match ctype {
            MimeType::ApplicationJson => {
                let j: Value = from_str(t.as_str()).unwrap();

                RequestBody{content: RequestBodyType::Json(j), content_type: ctype}
            },
            _ => RequestBody{content: RequestBodyType::Str(t), content_type: ctype},
        }
    }

}

#[derive(Clone)]
pub enum ResponseBodyType<'a, T: Clone + ResponseCommon + Deserialize<'a>> {
    Object(T),
    Str(String),
    PhantomData(&'a T),
}

pub trait ResponseCommon {
    fn parse_to_string(&self) -> String;
    fn get_length(&self) -> usize;
}

pub struct ResponseBody<'a, T: Clone + ResponseCommon + Deserialize<'a>> {
    pub content_type: MimeType,
    pub content: ResponseBodyType<'a, T>,
    _phantom: PhantomData<&'a T>
}

impl<'a,T: Clone + ResponseCommon + Deserialize<'a>> ResponseBody<'a, T> {
    pub fn new_json(content: T) -> Self {       
        ResponseBody{content: ResponseBodyType::Object(content), 
            content_type: MimeType::ApplicationJson, 
            _phantom: PhantomData}            
    }

    pub fn new_string(ctype: MimeType, t: String) -> Self {
        match ctype {
            MimeType::ApplicationJson => panic!("Wrong mimetype"),
            _ => ResponseBody {content_type: ctype, 
                                content: ResponseBodyType::Str(t),
                                _phantom: PhantomData},
        }
    }
}

#[derive(Clone)]
pub struct Params {
    pub key: String,
    pub value: String,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
    OPTION,
    PUT,
    DELETE,
    UNKNOWN,
}

impl From<String> for Method {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "get" => Method::GET,
            "post" => Method::POST,
            "option" => Method::OPTION,
            "put" => Method::PUT,
            "delete" => Method::DELETE,
            _ => Method::UNKNOWN,
         }
    }
}

impl Into<String> for Method {
    fn into(self) -> String {
        match self {
            Method::GET => "GET".to_string(),
            Method::POST => "POST".to_string(),
            Method::OPTION => "OPTION".to_string(),
            Method::PUT => "PUT".to_string(),
            Method::DELETE => "DELETE".to_string(),
            Method::UNKNOWN => "UNKNOW".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
}


#[derive(Clone, Copy)]
pub enum HttpStatus {
    Http100Continue,
    Http101SwitchingProtocols,
    Http102Processing,
    Http103EarlyHints,
    Http200Ok,
    Http201Created,
    Http202Accepted,
    Http203NonAuthorative,
    Http204NoContent,
    Http205ResetContent,
    Http206PartialContent,
    Http207Multi,
    Http208AlreadyReported,
    Http226IMUsed,
    Http300MultipleChoices,
    Http301MoovedPermenantly,
    Http302Found,
    Http303SeeOther,
    Http304NotModified,
    Http305UseProxy,
    Http306SwitchProxy,
    Http307TemporaryRedirect,
    Http308PermanentRedirect,
    Http400BadRequest,
    Http401Unauthorized,
    Http402PaymentRequired,
    Http403Forbidden,
    Http404NotFound,
    Http405MethodNotAllwed,
    Http406NotAcceptable,
    Http407ProxyAuthRequired,
    Http408RequestTimeout,
    Http409Conflict,
    Http410Gone,
    Http411LengthRequired,
    Http412PreconditionFailed,
    Http413PayloadTooLarge,
    Http414URITooLong,
    Http415UnsupportedMediaType,
    Http416RangeNotSatisfiable,
    Http417ExcpectationFailed,
    Http418ImATeapot,
    Http421MisdirectedRequest,
    Http422UnprocessableEntity,
    Http423Locked,
    Http424FailedDependency,
    Http425TooEarly,
    Http426UpgradeRequired,
    Http428PreconditionRequired,
    Http429TooManyRequests,
    Http431RequestHeaderFieldsTooLarge,
    Http451UnavailableForLegalReasons,
    Http500InternalServerError,
    Http501NotImplemented,
    Http502BadGateway,
    Http503ServiceUnavailable,
    Http504GatewayTimeout,
    Http505HTTPVersionNotSupported,
    Http506VariantAlsoNegotiates,
    Http507InsufficientStorage,
    Http508LoopDetected,
    Http510NotExtended,
    Http511NetworkAuthenticationRequried,

}

impl From<u32> for HttpStatus {
    fn from(i: u32) -> Self {
        match i {
            100 => HttpStatus::Http100Continue,
            101 => HttpStatus::Http101SwitchingProtocols,
            102 => HttpStatus::Http102Processing,
            103 => HttpStatus::Http103EarlyHints,
            200 => HttpStatus::Http200Ok,
            201 => HttpStatus::Http201Created,
            202 => HttpStatus::Http202Accepted,
            203 => HttpStatus::Http203NonAuthorative,
            204 => HttpStatus::Http204NoContent,
            205 => HttpStatus::Http205ResetContent,
            206 => HttpStatus::Http206PartialContent,
            207 => HttpStatus::Http207Multi,
            208 => HttpStatus::Http208AlreadyReported,
            226 => HttpStatus::Http226IMUsed,
            300 => HttpStatus::Http300MultipleChoices,
            301 => HttpStatus::Http301MoovedPermenantly,
            302 => HttpStatus::Http302Found,
            303 => HttpStatus::Http303SeeOther,
            304 => HttpStatus::Http304NotModified,
            305 => HttpStatus::Http305UseProxy,
            306 => HttpStatus::Http306SwitchProxy,
            307 => HttpStatus::Http307TemporaryRedirect,
            308 => HttpStatus::Http308PermanentRedirect,
            400 => HttpStatus::Http400BadRequest,
            401 => HttpStatus::Http401Unauthorized,
            402 => HttpStatus::Http402PaymentRequired,
            403 => HttpStatus::Http403Forbidden,
            404 => HttpStatus::Http404NotFound,
            405 => HttpStatus::Http405MethodNotAllwed,
            406 => HttpStatus::Http406NotAcceptable,
            407 => HttpStatus::Http407ProxyAuthRequired,
            408 => HttpStatus::Http408RequestTimeout,
            409 => HttpStatus::Http409Conflict,
            410 => HttpStatus::Http410Gone,
            411 => HttpStatus::Http411LengthRequired,
            412 => HttpStatus::Http412PreconditionFailed,
            413 => HttpStatus::Http413PayloadTooLarge,
            414 => HttpStatus::Http414URITooLong,
            415 => HttpStatus::Http415UnsupportedMediaType,
            416 => HttpStatus::Http416RangeNotSatisfiable,
            417 => HttpStatus::Http417ExcpectationFailed,
            418 => HttpStatus::Http418ImATeapot,
            421 => HttpStatus::Http421MisdirectedRequest,
            422 => HttpStatus::Http422UnprocessableEntity,
            423 => HttpStatus::Http423Locked,
            424 => HttpStatus::Http424FailedDependency,
            425 => HttpStatus::Http425TooEarly,
            426 => HttpStatus::Http426UpgradeRequired,
            428 => HttpStatus::Http428PreconditionRequired,
            429 => HttpStatus::Http429TooManyRequests,
            431 => HttpStatus::Http431RequestHeaderFieldsTooLarge,
            451 => HttpStatus::Http451UnavailableForLegalReasons,
            500 => HttpStatus::Http500InternalServerError,
            501 => HttpStatus::Http501NotImplemented,
            502 => HttpStatus::Http502BadGateway,
            503 => HttpStatus::Http503ServiceUnavailable,
            504 => HttpStatus::Http504GatewayTimeout,
            505 => HttpStatus::Http505HTTPVersionNotSupported,
            506 => HttpStatus::Http506VariantAlsoNegotiates,
            507 => HttpStatus::Http507InsufficientStorage,
            508 => HttpStatus::Http508LoopDetected,
            510 => HttpStatus::Http510NotExtended,
            511 => HttpStatus::Http511NetworkAuthenticationRequried,
            _ => panic!("No such status in HTTP, see https://en.wikipedia.org/wiki/List_of_HTTP_status_codes")
    
        }
    }
}


impl Into<String> for HttpStatus {
    fn into(self) -> String {
        match self {
            HttpStatus::Http100Continue => "100 Continue".to_string(),
            HttpStatus::Http101SwitchingProtocols => "101 Switching Protocols".to_string(),
            HttpStatus::Http102Processing => "102 Processing".to_string(),
            HttpStatus::Http103EarlyHints => "103 Early Hints".to_string(),
            HttpStatus::Http200Ok => "200 OK".to_string(),
            HttpStatus::Http201Created => "201 Created".to_string(),
            HttpStatus::Http202Accepted => "202 Accepted".to_string(),
            HttpStatus::Http203NonAuthorative => "203 Non-Authoritative Information".to_string(),
            HttpStatus::Http204NoContent => "204 No Content".to_string(),
            HttpStatus::Http205ResetContent => "205 Reset Content".to_string(),
            HttpStatus::Http206PartialContent => "206 Partial Content".to_string(),
            HttpStatus::Http207Multi => "207 Multi-Status".to_string(),
            HttpStatus::Http208AlreadyReported => "208 Already Reported".to_string(),
            HttpStatus::Http226IMUsed => "226 IM Used".to_string(),
            HttpStatus::Http300MultipleChoices => "300 Multiple Choices".to_string(),
            HttpStatus::Http301MoovedPermenantly => "301 Moved Permanently".to_string(),
            HttpStatus::Http302Found => "302 Found".to_string(),
            HttpStatus::Http303SeeOther => "303 See Other".to_string(),
            HttpStatus::Http304NotModified => "304 Not Modified".to_string(),
            HttpStatus::Http305UseProxy => "305 Use Proxy".to_string(),
            HttpStatus::Http306SwitchProxy => "306 Switch Proxy".to_string(),
            HttpStatus::Http307TemporaryRedirect => "307 Temporary Redirect".to_string(),
            HttpStatus::Http308PermanentRedirect => "308 Permanent Redirect".to_string(),
            HttpStatus::Http400BadRequest => "400 Bad Request".to_string(),
            HttpStatus::Http401Unauthorized => "401 Unauthorized".to_string(),
            HttpStatus::Http402PaymentRequired => "402 Payment Required".to_string(),
            HttpStatus::Http403Forbidden => "403 Forbidden".to_string(),
            HttpStatus::Http404NotFound => "404 Not Found".to_string(),
            HttpStatus::Http405MethodNotAllwed => "405 Method Not Allowed".to_string(),
            HttpStatus::Http406NotAcceptable => "406 Not Acceptable".to_string(),
            HttpStatus::Http407ProxyAuthRequired => "407 Proxy Authentication Required".to_string(),
            HttpStatus::Http408RequestTimeout => "408 Request Timeout".to_string(),
            HttpStatus::Http409Conflict => "409 Conflict".to_string(),
            HttpStatus::Http410Gone => "410 Gone".to_string(),
            HttpStatus::Http411LengthRequired => "411 Length Required".to_string(),
            HttpStatus::Http412PreconditionFailed => "412 Precondition Failed".to_string(),
            HttpStatus::Http413PayloadTooLarge => "413 Payload Too Large".to_string(),
            HttpStatus::Http414URITooLong => "414 URI Too Long".to_string(),
            HttpStatus::Http415UnsupportedMediaType => "415 Unsupported Media Type".to_string(),
            HttpStatus::Http416RangeNotSatisfiable => "416 Range Not Satisfiable".to_string(),
            HttpStatus::Http417ExcpectationFailed => "417 Expectation Failed".to_string(),
            HttpStatus::Http418ImATeapot => "418 I'm a teapot".to_string(),
            HttpStatus::Http421MisdirectedRequest => "421 Misdirected Request".to_string(),
            HttpStatus::Http422UnprocessableEntity => "422 Unprocessable Entity".to_string(),
            HttpStatus::Http423Locked => "423 Locked".to_string(),
            HttpStatus::Http424FailedDependency => "424 Failed Dependency".to_string(),
            HttpStatus::Http425TooEarly => "425 Too Early".to_string(),
            HttpStatus::Http426UpgradeRequired => "426 Upgrade Required".to_string(),
            HttpStatus::Http428PreconditionRequired => "428 Precondition Required".to_string(),
            HttpStatus::Http429TooManyRequests => "429 Too Many Requests".to_string(),
            HttpStatus::Http431RequestHeaderFieldsTooLarge => "431 Request Header Fields Too Large".to_string(),
            HttpStatus::Http451UnavailableForLegalReasons => "451 Unavailable For Legal Reasons".to_string(),
            HttpStatus::Http500InternalServerError => "500 Internal Server Error".to_string(),
            HttpStatus::Http501NotImplemented => "501 Not Implemented".to_string(),
            HttpStatus::Http502BadGateway => "502 Bad Gateway".to_string(),
            HttpStatus::Http503ServiceUnavailable => "503 Service Unavailable".to_string(),
            HttpStatus::Http504GatewayTimeout => "504 Gateway Timeout".to_string(),
            HttpStatus::Http505HTTPVersionNotSupported => "505 HTTP Version Not Supported".to_string(),
            HttpStatus::Http506VariantAlsoNegotiates => "506 Variant Also Negotiates".to_string(),
            HttpStatus::Http507InsufficientStorage => "507 Insufficient Storage".to_string(),
            HttpStatus::Http508LoopDetected => "508 Loop Detected".to_string(),
            HttpStatus::Http510NotExtended => "510 Not Extended".to_string(),
            HttpStatus::Http511NetworkAuthenticationRequried => "511 Network Authentication Required".to_string(),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct DummyResponseType;

impl ResponseCommon for DummyResponseType {
    fn parse_to_string(&self) -> String {
        String::new()
    }

    fn get_length(&self) -> usize {
        0usize
    }
}



pub struct ResponseTextWrapper(String);

impl ResponseTextWrapper {
    pub fn new(t: String) -> Self {
        ResponseTextWrapper(t)
    }

    pub fn serve(&self, stream: &mut TcpStream) {
        let ResponseTextWrapper(t) = self;

        stream.write_all(t.as_bytes()).unwrap();

        stream.shutdown(Shutdown::Both).unwrap();

        println!("Served!");
    }
}

pub struct TemplateHolder {
    template: String,
}

impl TemplateHolder {
    pub fn new(t: String) -> Self {
        TemplateHolder { template: t }
    }

    pub fn mutate(&mut self, t_rep: String) {
        self.template = t_rep;
    }

    pub fn get_temp(&self) -> String {
        self.template.clone()
    }
}



lazy_static! {
    pub static ref TEMP_404: Mutex<TemplateHolder>  = Mutex::new(TemplateHolder::new("404 Not Found".to_string()));
    pub static ref TEMP_405: Mutex<TemplateHolder>  = Mutex::new(TemplateHolder::new("405 Not Found".to_string()));
}




pub fn modify_404_template(template: String) {
    let mut temp_lock = TEMP_404.lock().unwrap();

    temp_lock.mutate(template)
}

pub fn modify_405_template(template: String) {
    let mut temp_lock = TEMP_405.lock().unwrap();

    temp_lock.mutate(template)
}