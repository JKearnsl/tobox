use std::{io, thread};
use std::net::TcpListener;

pub struct ConnectionConfig {
    pub tcp_listener: Option<TcpListener>,
    pub workers: usize,
    pub tls: Option<(String, String)>,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            tcp_listener: None,
            workers: match thread::available_parallelism() {
                Ok(parallelism) => usize::from(parallelism),
                Err(_) => 1,
            },
            tls: None,
        }
    }
}


pub trait Server {
    fn bind(self, addr: String) -> io::Result<Self> where Self: Sized;
    fn set_workers(self, workers: usize) -> Self where Self: Sized;
    fn set_tls(self, key: &str, cert: &str) -> Self where Self: Sized;
    fn run(self) -> io::Result<()> where Self: Sized;
}
