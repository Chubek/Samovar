use crate::{
    common::{Method, ResponseTextWrapper},
    context::{ContextOp, ContextType},
    endpoint::Endpoint,
    request::Request,
    staticserver::{DirServer, StaticServerType},
};
use std::ops::Deref;
use std::{
    any::Any,
    collections::HashMap,
    net::TcpListener,
    sync::{Arc, Mutex},
};

pub struct Samovar {
    port: u32,
    address: &'static str,
    context: HashMap<String, Arc<Mutex<dyn Any + Send>>>,
    listener: TcpListener,
    static_servers: Vec<StaticServerType>,
}

impl Samovar {
    pub fn new(address: &'static str, port: u32) -> Self {
        let listener = TcpListener::bind(format!("{}:{}", address, port)).unwrap();
        let context = HashMap::<String, Arc<Mutex<dyn Any + Send>>>::new();
        let static_servers: Vec<StaticServerType> = vec![];

        Samovar {
            listener,
            address,
            port,
            context,
            static_servers,
        }
    }

    pub fn insert_context(&mut self, key: String, ctx: Arc<Mutex<dyn Any + Send>>) {
        self.context.insert(key, ctx);
    }

    pub fn insert_static_server(&mut self, server_type: StaticServerType) {
        self.static_servers.push(server_type);
    }

    fn construct_static_server_single(&mut self, server_type: &StaticServerType) {
        let (glob_path, serve_index) = {
            match server_type {
                StaticServerType::ServeWithIndex(p) => {
                    let serve_index = true;
                    let glob_path = p.clone();

                    (glob_path, serve_index)
                }
                StaticServerType::ServeWithoutIndex(p) => {
                    let serve_index = true;
                    let glob_path = p.clone();

                    (glob_path, serve_index)
                }
            }
        };

        let dir_server = DirServer::new(&glob_path, serve_index);

        let uri_path = &dir_server.get_path();

        let dir_server_name = dir_server.compose_name();

        let dir_server_ref_cell = Arc::new(Mutex::new(Box::new(dir_server)));

        self.insert_context(dir_server_name, dir_server_ref_cell);

        fn serve_dir_server(req: &Request, ctx: &ContextType) -> ResponseTextWrapper {
            let req_name_fs = req.compose_name_for_fs();

            let context_deref = ctx.deref();

            match context_deref.get(&req_name_fs) {
                Some(c) => {
                    let deref = c.deref();

                    let mut locked = Box::new(deref.lock().unwrap());

                    let res = locked
                        .downcast_mut::<Box<dyn ContextOp<ResponseTextWrapper, Vec<String>>>>()
                        .unwrap();

                    res.op(vec![req.uri.clone()])
                }
                None => ResponseTextWrapper::new("ERR".to_string()),
            }
        }

        let callable = &serve_dir_server;
        let method = Method::GET;

        let v = Endpoint::new(uri_path.clone(), callable, method);

        let mut lock_map = crate::common::ENDPOINT_MAP.lock().unwrap();

        let k = uri_path.replace("/", "_");

        lock_map.insert(k, v);
    }

    pub fn construct_static_servers(&mut self) {
        let _ = self
            .static_servers
            .clone()
            .into_iter()
            .map(|x| self.construct_static_server_single(&x));
    }
}
