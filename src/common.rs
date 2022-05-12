use std::marker::PhantomData;

use serde_json::from_str;
use serde::{Serialize, Deserialize};

pub struct Header {
    pub key: String,
    pub value: String,
}

pub enum MimeType {
    ApplicationJson,
    TextPlain,
    TextHtml,
    ApplicationOctetStream,
}

pub struct Body<'a, T: BodyType + Serialize + Deserialize<'a> + std::convert::From<&'a String>> {
    pub content_type: MimeType,
    pub content: T,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T: BodyType + Serialize + Deserialize<'a> + std::convert::From<&'a String>> Body<'a, T> {
    fn from_str(t: &'a String, ctype: self::MimeType) -> Self {
        let _phantom = PhantomData;
        
        match ctype {
            MimeType::ApplicationJson => {
                let b: T = from_str(t.as_str()).unwrap();

                Body { content_type: ctype, content: b, _phantom}                  
            },
            _ => { 
                let b: T = t.into();
                Body {content_type: ctype,  content: b, _phantom}}

           
        }
    }
}

pub struct Params {
    pub key: String,
    pub value: String,
}

pub enum Method {
    GET,
    POST,
    OPTION,
    PUT,
    DELETE,
}

pub struct UserInfo {
    pub username: String,
    pub password: String,
}


pub trait BodyType {

}

pub enum HttpStatus {
    Http100Continue,
    Http101Switch,
    Http102Processing,
    Http103EarlyHints,
    Http200Ok,
    Http201Created,
    Http203NonAuthorative,
    Http204NoContent,
    Http205ResetContent,
    Http206PartialContent,
    Http207Multi,
    Http208AlreadyReported,
    Http226IMUsed,
    Http300MultipleChoices,
    Http301MoovedPermenantly,
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