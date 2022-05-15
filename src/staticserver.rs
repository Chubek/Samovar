use crate::response::Response;
use crate::{common::*, context::ContextOp};
use content_inspector::inspect;
use glob::glob;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

lazy_static! {
    static ref HTML_SERVER: &'static str = r#"<html data-theme="coffee"><head><title>Chuby-HTTP FileServer --- Powered by Ritalin</title><link rel="preconnect" href="https://fonts.googleapis.com"><link rel="preconnect" href="https://fonts.gstatic.com" crossorigin><link href="https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap" rel="stylesheet"><link href="https://cdn.jsdelivr.net/npm/daisyui@2.14.3/dist/full.css" rel="stylesheet" type="text/css"/><script src="https://cdn.tailwindcss.com"></script><link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/tailwindcss/2.2.19/tailwind.min.css" integrity="sha512-wnea99uKIC3TJF7v4eKk4Y+lMz2Mklv18+r4na2Gn1abDRPPOeef95xTzdwGD9e6zXJBteMIhZ1+68QC5byJZw==" crossorigin="anonymous" referrerpolicy="no-referrer"/></head><body><div class="flex flex-col place-items-center m-5 border-opacity-50">REPLACE_ALL</div><div class="grid h-20 w-full m-3 card bg-base-200 rounded-box place-items-center footer"><div class="footer-center">Powered by Chuby-HTTPFind me on Github: <a href="https://github.com/chbuek">github.com/chubek</a></div></div></html>"#;
    static ref HTML_FILE: &'static str = r#" <div class="grid h-20 w-3/5 m-3 card bg-base-300 rounded-box place-items-center"> <span class="indicator-item badge badge-secondary">REPLACE_BADGE</span> <a class="link" href="REPLACE_HREF">REPLACE_NAME</a> </div>"#;
    static ref BADGE_MAP: HashMap<MimeType, &'static str> = {
        let mut m = HashMap::<MimeType, &'static str>::new();

        m.insert(MimeType::ApplicationJson, "badge-primary");
        m.insert(MimeType::TextJavaScript, "badge-secondary");
        m.insert(MimeType::TextPlain, "badge-warning");
        m.insert(MimeType::TextCSS, "bade-error");
        m.insert(MimeType::ApplicationOctetStream, "badge-accent");
        m.insert(MimeType::TextHtml, "badge-info");

        m
    };
}

#[derive(Clone)]
struct FileCache {
    path: PathBuf,
    mimetype: MimeType,
    content: String,
    is_index: bool,
    uri: String,
}

#[derive(Clone)]
pub struct DirServer {
    path: String,
    cache: Vec<FileCache>,
    serve_index: bool,
}

impl DirServer {
    pub fn new(glob_path: &str, serve_index: bool) -> Self {
        let mut cache: Vec<FileCache> = vec![];
        let mut has_index = false;

        for entry in glob(glob_path).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    let content = read_to_string(path.clone()).unwrap();
                    let is_text = inspect(content.clone().as_bytes()).is_text();

                    let mimetype = Self::guess_mime(path.clone(), is_text.clone());
                    let is_index = Self::is_index(path.clone(), is_text.clone());
                    let uri = Self::get_uri(path.clone());

                    let fc = FileCache {
                        path,
                        mimetype,
                        content,
                        uri,
                        is_index,
                    };

                    cache.push(fc);

                    has_index = is_index;
                }
                Err(e) => println!("{:?}", e),
            }
        }

        let mut path = Path::new(glob_path)
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        path = path.replace("/*", "");
        path = path.replace("*/", "");
        path = path.replace("*", "");

        match has_index {
            true => DirServer {
                cache,
                path,
                serve_index,
            },
            false => DirServer {
                cache,
                path,
                serve_index: false,
            },
        }
    }

    pub fn get_path(&self) -> String {
        self.path.clone().to_owned()
    }

    fn guess_mime(path: PathBuf, is_text: bool) -> MimeType {
        match path.extension() {
            Some(ext) => {
                if let Some(ext_str) = ext.to_str() {
                    match ext_str {
                        "js" => MimeType::TextJavaScript,
                        "css" => MimeType::TextCSS,
                        "json" => MimeType::ApplicationJson,
                        "html" => MimeType::TextHtml,
                        _ => match is_text {
                            true => MimeType::TextPlain,
                            false => MimeType::ApplicationOctetStream,
                        },
                    }
                } else {
                    panic!("Error parsing osstr")
                }
            }
            None => match is_text {
                true => MimeType::TextPlain,
                false => MimeType::ApplicationOctetStream,
            },
        }
    }

    fn is_index(path: PathBuf, is_text: bool) -> bool {
        match path.file_stem() {
            Some(fname) => {
                if let Some(name_str) = fname.to_str() {
                    if name_str == "index" {
                        if is_text {
                            return true;
                        } else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            None => todo!(),
        }
    }

    fn get_uri(path: PathBuf) -> String {
        let relative_path = path.parent().unwrap().to_str().unwrap();
        let fname = path.file_name().unwrap().to_str().unwrap();

        let replaced = relative_path.replace("\\", "/");

        format!("{}{}", replaced, fname)
    }

    fn create_file_item(item: &FileCache) -> String {
        let copy_str = HTML_FILE.clone().to_string();

        let fname = item.path.file_name().unwrap().to_str().unwrap();
        let badge_color = BADGE_MAP[&item.mimetype];

        let mut replaced = copy_str.replace("REPLACE_HREF", &item.uri);
        replaced = replaced.replace("REPLACE_BADGE", badge_color);
        replaced = replaced.replace("REPLACE_NAME", fname);

        replaced
    }

    fn create_file_list(&self) -> String {
        let files_joined = self
            .cache
            .iter()
            .map(|x| Self::create_file_item(x))
            .collect::<Vec<String>>()
            .join(r#"<div class="divider"></div>"#);
        let html_temp_copy = HTML_SERVER.clone().to_string();

        let replace = html_temp_copy.replace("REPLACE_ALL", &files_joined);

        replace
    }

    fn crease_response_with_file(&self, uri: String) -> ResponseTextWrapper {
        let mut cache_clone = self.cache.clone();

        cache_clone.retain(|x| x.uri == uri);

        let item = &cache_clone[0];

        let mut response = Response::<DummyResponseType>::new_string(
            item.content.clone(),
            item.mimetype.clone(),
            HttpStatus::Http200Ok,
        );

        let response_text = response.compose();

        response_text
    }

    fn create_respons_with_index(&self) -> ResponseTextWrapper {
        let list = self.create_file_list();

        let mut response = Response::<DummyResponseType>::new_string(
            list,
            MimeType::TextHtml,
            HttpStatus::Http200Ok,
        );

        let response_text = response.compose();

        response_text
    }

    fn create_response_with_index_file(&self) -> ResponseTextWrapper {
        let mut cache_clone = self.cache.clone();

        cache_clone.retain(|x| x.is_index);

        let item = &cache_clone[0];

        let mut response = Response::<DummyResponseType>::new_string(
            item.content.clone(),
            item.mimetype.clone(),
            HttpStatus::Http200Ok,
        );

        let response_text = response.compose();

        response_text
    }

    pub fn compose_name(&self) -> String {
        let name = format!("dir_server_{}", self.path.replace("/", "-"));

        name
    }

    pub fn is_uri_server(&self, uri: &String) -> bool {
        let is_in = uri.contains(self.path.as_str());

        is_in
    }

    pub fn compose(&self, uri: String) -> ResponseTextWrapper {
        match uri == self.path {
            true => match self.serve_index {
                true => self.create_response_with_index_file(),
                false => self.create_respons_with_index(),
            },
            false => self.crease_response_with_file(uri),
        }
    }
}

pub struct SingleFileServer(FileCache);

impl SingleFileServer {
    pub fn new(path: PathBuf) -> Self {
        let content = read_to_string(path.clone()).expect("File does not exist");
        let is_text = inspect(content.as_bytes()).is_text();
        let mimetype = DirServer::guess_mime(path.clone(), is_text);
        let is_index = false;
        let uri = DirServer::get_uri(path.clone());

        let fc = FileCache {
            content,
            mimetype,
            uri,
            is_index,
            path,
        };

        SingleFileServer(fc)
    }

    fn compose(&self) -> ResponseTextWrapper {
        let SingleFileServer(fc) = self;

        let mut response = Response::<DummyResponseType>::new_string(
            fc.content.clone(),
            fc.mimetype.clone(),
            HttpStatus::Http200Ok,
        );

        let response_text = response.compose();

        response_text
    }
}

#[derive(Clone)]
pub enum StaticServerType {
    ServeWithIndex(String),
    ServeWithoutIndex(String),
}

impl ContextOp<ResponseTextWrapper, Vec<String>> for DirServer {
    fn op(&self, args: Vec<String>) -> ResponseTextWrapper {
        let arg_0 = args.into_iter().next().unwrap();

        self.compose(arg_0)
    }
}
