#[macro_use]
extern crate imgui;

extern crate stratis_server as server;
extern crate stratis_shared as shared;

use server::{Server};
use shared::interface::Interface;

use std::thread;

#[allow(unused_must_use)]
fn main() {
    let ip_addr = "127.0.0.1:9996";
    let server = Server::new();
    let mut handler = server.clone();
    thread::spawn(move || {
        handler.listen(ip_addr);
    });

    let mut ifc = Interface::init("stratis server", [800,600]);
    
    'main: loop {
        if !ifc.render(None, |ui| {
            ui.window(im_str!("Players"))
                .build(|| {});
        }) { break 'main }
        else { ifc.maybe_sleep() }
    }
}
