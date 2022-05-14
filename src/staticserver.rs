use std::{path::PathBuf, fs::File};
use glob::glob;
use crate::common::*;
use std::fs::read_to_string;
use content_inspector::inspect;

struct FileCache {
    path: PathBuf,
    mimetype: MimeType,
    content: String,
    is_index: bool,
}

pub struct DirServer {
    path: String,
    cache: Vec<FileCache>,
}

impl DirServer {
    pub fn new(glob_path: &str) -> Self {
        let mut cache: Vec<FileCache> = vec![];
        let index_path = String::new();

        for entry in glob(glob_path).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) =>  {
                    let content = read_to_string(path).unwrap();
                    let is_text = inspect(content.clone().as_bytes()).is_text();

                    let mimetype = Self::guess_mime(path.clone(), is_text);

                    let fc = FileCache { path, mimetype, content };

                    cache.push(fc);

                },
                Err(e) => println!("{:?}", e),
            }
        }

        DirServer { cache }
        
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
                        _ => {
                            match is_text {
                                true => MimeType::TextPlain,
                                false => MimeType::ApplicationOctetStream
                            }
                        }
                    }
                } else {
                    panic!("Error parsing osstr")
                }
            },
            None => {
                match is_text {
                    true => MimeType::TextPlain,
                    false => MimeType::ApplicationOctetStream
                }
            },
        }
                
     
    }

    fn is_index(path: PathBuf) -> bool {
        path.
    }

}