use regex::Regex;

lazy_static! {
    static ref GET_PATT: Regex = Regex::new(r#"(?i)\bGET\b\s\/?[A-Za-z0-9._\-\?\/?]+\sHTTP\/[\d\.]+"#).unwrap();
    static ref POST_PATT: Regex = Regex::new(r#"(?i)\bPOST\b\s\/?[A-Za-z0-9._\-\?\/?]+\sHTTP\/[\d\.]+"#).unwrap();
    static ref OPTION_PATTT: Regex = Regex::new(r#"(?i)\bOPTIONS\b\s\/?[A-Za-z0-9._\-\?\/?]+\sHTTP\/[\d\.]+"#).unwrap();
    static ref PUT_PATT: Regex = Regex::new(r#"(?i)\bPUT\b\s\/?[A-Za-z0-9._\-\?\/?]+\sHTTP\/[\d\.]+"#).unwrap();
    static ref DELETE_PATT: Regex = Regex::new(r#"(?i)\bDELETE\b\s\/?[A-Za-z0-9._\-\?\/?]+\sHTTP\/[\d\.]+"#).unwrap();
    static ref URI_PATT: Regex = Regex::new(r#"(?i)(http(s?)://)?(\/?[0-9A-Za-z_.\-]+\/)+([0-9A-Za-z_.\-]+)(\?q=)?(([a-zA-Z0-9]+\=[a-zA-Z0-9]+)\&?)+"#).unwrap();
    static ref HEADER_PATT: Regex = Regex::new(r#"(?i)((?![host])([A-Za-z0-9\-\=\"\'\;\,\/\?]+)\s?)\:(\s?([A-Za-z0-9\-\=\"\'\;\,\s\/\?]+)\n)"#).unwrap();
    static ref HOST_PATT: Regex: Regex::new(r#"(?i)(host\s?)\:(\s[A-Za-z0-9\:\-\/\&\?\!\.]+)"#).unwrap();
    static ref REFERER_PATT: Regex: Regex::new(r#"(?i)(referer\s?)\:(\s[A-Za-z0-9\:\-\/\&\?\!\.]+)"#).unwrap();
}