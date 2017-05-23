extern crate stratis_server as server;
extern crate stratis_shared as shared;

use server::{Server};

fn main() {
    let ip_addr = "127.0.0.1:9996";
    let server = Server::new();
    let mut handler = server.clone();
    handler.listen(ip_addr); // NOTE: this fails silently if server is already running
}
