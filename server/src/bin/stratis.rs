extern crate stratis_server as server;

use server::{Server};

#[allow(unused_must_use)]
fn main() {
    let ip_addr = "127.0.0.1:9996";
    
    Server::new(ip_addr);
}
