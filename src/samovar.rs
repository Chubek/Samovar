use crate::context::Context;
use std::net::{TcpListener, TcpStream};

pub struct Samovar {
    port: u32,
    address: &'static str,
    context: Vec<Context>,
    listener: TcpListener,
}


impl Samovar {
    pub fn new(address: &'static str, port: u32) -> Self {
        let listener = TcpListener::bind(format!("{}:{}", address, port)).unwrap();
        let context: Vec<Context> = vec![];

        Samovar { listener, address, port, context }
    }

    pub fn inser_context(&mut self, ctx: Context) {
        self.context.push(ctx);
    }

}