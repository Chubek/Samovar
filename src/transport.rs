/* 
use std::thread;
use std::net::{TcpListener};

pub struct TcpTransporter(TcpListener);


impl TcpTransporter {
    pub fn new(addr: &str) -> Self {
        let tcp_listener = TcpListener::bind(addr).unwrap();

        TcpTransporter(tcp_listener)
    }

}
*/
