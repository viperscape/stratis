extern crate stratis_shared as shared;

use shared::client::{Client};
use shared::events::Event;
use shared::interface::Interface;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;


#[allow(unused_must_use)]
fn main() {    
    let ip_addr = "127.0.0.1:9996";
    
    let client;
    let rx: Receiver<Event>;
    if let Some((mut c, rx_)) = Client::load_file("client.key") {
        c.connect(ip_addr);
        client = Arc::new(Mutex::new(c));
        rx = rx_;
        Client::login(&client);
    }
    else {
        let (mut c, rx_) = Client::default();
        Client::save(&c,"client.key");
        c.connect(ip_addr);
        c.register();
        client = Arc::new(Mutex::new(c));
        rx = rx_;
        Client::login(&client);
    }

    Client::handler(client.clone());

    let mut ifc = Interface::init("stratis client", [800,600]);
    
    'main: loop {
        if !ifc.render(None, |_ui| {
            //ui.show_test_window(&mut true) // NOTE: use for imgui examples
        }) { break 'main }
        else { ifc.maybe_sleep() }
    }
}
