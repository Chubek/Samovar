use crate::common::*;
use crate::response::Response;
use content_inspector::inspect;
use glob::glob;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;

lazy_static! {
    static ref HTML_SERVER: &'static str = r#"<html data-theme="coffee"><head><title>Chuby-HTTP FileServer --- Powered by Ritalin</title><link rel="preconnect" href="https://fonts.googleapis.com"><link rel="preconnect" href="https://fonts.gstatic.com" crossorigin><link href="https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap" rel="stylesheet"><link href="https://cdn.jsdelivr.net/npm/daisyui@2.14.3/dist/full.css" rel="stylesheet" type="text/css"/><script src="https://cdn.tailwindcss.com"></script><link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/tailwindcss/2.2.19/tailwind.min.css" integrity="sha512-wnea99uKIC3TJF7v4eKk4Y+lMz2Mklv18+r4na2Gn1abDRPPOeef95xTzdwGD9e6zXJBteMIhZ1+68QC5byJZw==" crossorigin="anonymous" referrerpolicy="no-referrer"/></head><body><div class="flex flex-col place-items-center m-5 border-opacity-50">REPLACE_ALL</div><div class="grid h-20 w-full m-3 card bg-base-200 rounded-box place-items-center footer"><div class="footer-center">Powered by Samovar<br>Find me on Github: <a href="https://github.com/chbuek">github.com/chubek</a></div></div></html>"#;
    static ref HTML_FILE: &'static str = r#" <div class="grid h-20 w-3/5 m-3 card bg-base-300 rounded-box place-items-center"> <span class="indicator-item badge REPLACE_BADGE">REPLACE_TYPE</span> <a class="link" href="REPLACE_HREF">REPLACE_NAME</a> </div>"#;
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
    uri: String,
}

#[derive(Clone)]
pub struct DirServer {
    path_uri: String,
    cache: Vec<FileCache>,
    index_file: Option<String>,
}

impl DirServer {
    pub fn new(path_uri: String, glob_path: &str, index_file: Option<String>) -> Self {
        let mut cache: Vec<FileCache> = vec![];

        let mut path_uri_chars = path_uri.chars().collect::<Vec<char>>();

        if path_uri_chars.len() > 2 {
            if path_uri_chars.get(0).unwrap() == &'/' {
                path_uri_chars.remove(0);
            }

            if path_uri_chars.last().unwrap() == &'/' {
                path_uri_chars.pop();
            }
        }

        let path_fin = String::from_iter(path_uri_chars.iter());

        for entry in glob(glob_path).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    println!("Found file {}", path.display());

                    let content = read_to_string(path.clone()).unwrap();
                    let is_text = inspect(content.clone().as_bytes()).is_text();

                    let mimetype = Self::guess_mime(path.clone(), is_text.clone());
                    let uri = Self::get_uri(path_fin.clone(), path.clone());
                    println!("Got file URI: {}", &uri);
                    let fc = FileCache {
                        path,
                        mimetype,
                        content,
                        uri,
                    };

                    cache.push(fc);
                }
                Err(e) => println!("{:?}", e),
            }
        }

        println!("Found {} files in this glob", &cache.len());

        DirServer {
            cache,
            path_uri: path_fin,
            index_file,
        }
    }

    pub fn get_path(&self) -> String {
        self.path_uri.clone().to_owned()
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

    fn get_uri(path_uri: String, path: PathBuf) -> String {
        let fname = path.file_name().unwrap().to_str().unwrap();

        format!("{}/{}", path_uri, fname)
    }

    fn create_file_item(item: &FileCache) -> String {
        let copy_str = HTML_FILE.clone().to_string();

        let fname = item.path.file_name().unwrap().to_str().unwrap();
        let badge_color = BADGE_MAP[&item.mimetype];
        let ftype: String = item.mimetype.clone().into();

        let mut replaced = copy_str.replace("REPLACE_HREF", &item.uri);
        replaced = replaced.replace("REPLACE_BADGE", badge_color);
        replaced = replaced.replace("REPLACE_NAME", fname);
        replaced = replaced.replace("REPLACE_TYPE", &ftype);

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

    fn create_response_with_index(&self) -> ResponseTextWrapper {
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
        let fc_index = self.get_index_file();

        let fc_str = fc_index.content.clone();
        let fc_type = fc_index.mimetype.clone();

        let mut response =
            Response::<DummyResponseType>::new_string(fc_str, fc_type, HttpStatus::Http200Ok);

        let response_text = response.compose();

        response_text
    }

    fn get_index_file(&self) -> &FileCache {
        let mut ret = self.cache.iter().next().unwrap();

        for fc in &self.cache {
            let fname = fc.path.file_name().unwrap().to_str().unwrap();

            let index_name = self.index_file.clone().unwrap();

            if fname == index_name {
                ret = fc
            }
        }

        ret
    }

    pub fn compose_name(&self) -> String {
        let name = format!("dir_server_{}", self.path_uri.replace("/", "-"));

        name
    }

    pub fn is_uri_server(&self, uri: &String) -> bool {
        let is_in = uri.contains(self.path_uri.as_str());

        is_in
    }

    pub fn respond_empty(&self) -> ResponseTextWrapper {
        let mut resp = Response::<DummyResponseType>::new_string(
            "No files in this glob".to_string(),
            MimeType::TextPlain,
            HttpStatus::Http404NotFound,
        );

        resp.compose()
    }

    pub fn compose(&self, uri: String) -> ResponseTextWrapper {
        println!("Serving static file on uri {}", &uri);

        if self.cache.len() == 0 {
            println!("Empty glob path");
            return self.respond_empty();
        }

        match uri == self.path_uri {
            true => match self.index_file {
                Some(_) => self.create_response_with_index_file(),
                None => {
                    println!("Serve index is false, serving with file index...");
                    self.create_response_with_index()
                }
            },
            false => self.crease_response_with_file(uri),
        }
    }
}

pub struct SingleFileServer(FileCache);

impl SingleFileServer {
    pub fn new(path: PathBuf, path_uri: String) -> Self {
        let content = read_to_string(path.clone()).expect("File does not exist");
        let is_text = inspect(content.as_bytes()).is_text();
        let mimetype = DirServer::guess_mime(path.clone(), is_text);
        let uri = DirServer::get_uri(path_uri, path.clone());

        let fc = FileCache {
            content,
            mimetype,
            uri,
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
