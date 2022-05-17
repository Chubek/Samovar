use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

pub struct MemorySession {
    items: HashMap<String, String>,
}

impl MemorySession {
    pub fn new() -> Self {
        let items = HashMap::<String, String>::new();

        MemorySession { items }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.items.insert(key, value);
    }

    pub fn get(&self, key: &String) -> Option<String> {
        match self.items.get(key) {
            Some(t) => Some(t.clone()),
            None => None,
        }
    }
}

pub struct PhysicalSession {
    filepath: PathBuf,
    items: HashMap<String, String>,
}

impl PhysicalSession {
    pub fn new(fpath: String) -> Self {
        let filepath = PathBuf::from(fpath.as_str());
        let items = HashMap::<String, String>::new();

        PhysicalSession { items, filepath }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.items.insert(key, value);
        self.write();
    }

    pub fn get(&self, key: &String) -> Option<String> {
        match self.items.get(key) {
            Some(t) => Some(t.clone()),
            None => None,
        }
    }

    fn write(&self) {
        let mut f = File::create(&self.filepath).unwrap();
        let contents = self.compose();
        f.write_all(contents.as_bytes()).unwrap();
        f.sync_all().unwrap();
    }

    pub fn initiate(&mut self) {
        match self.filepath.exists() {
            true => {
                let mut f = File::open(&self.filepath).unwrap();

                let mut contents = String::new();

                f.read_to_string(&mut contents).unwrap();

                self.decompose(contents)
            }
            false => {
                let prefix = self.filepath.parent().unwrap();

                if !prefix.exists() {
                    std::fs::create_dir_all(prefix).unwrap();
                }

                File::create(&self.filepath).unwrap();
            }
        }
    }

    fn compose(&self) -> String {
        let mut ret: Vec<String> = vec![];

        for (k, v) in self.items.iter() {
            let line = format!("{} +/+/+ {}", k, v);

            ret.push(line);
        }

        let ret_joined = ret.join("/");

        ret_joined
    }

    fn decompose(&mut self, str: String) {
        let lines = str.lines().collect::<Vec<&str>>();

        for l in lines {
            let mut split = l.split(" +/+/+ ");

            let key_next = split.next().unwrap();
            let value_last = split.last().unwrap();

            let key = key_next.trim().to_string();
            let value = value_last.trim().to_string();

            self.insert(key, value);
        }
    }
}
