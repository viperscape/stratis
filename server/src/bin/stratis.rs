#[macro_use]
extern crate imgui;

extern crate stratis_server as server;
extern crate stratis_shared as shared;

use server::{Server};
use shared::client::{Client};
use shared::events::Event;
use shared::interface::Interface;

use imgui::{Ui,ImString,ImStr};

use std::thread;
use std::sync::mpsc::{Receiver};
use std::sync::{Arc, Mutex};

#[allow(unused_must_use,unused_variables,unused_assignments)]
fn main() {
    let ip_addr = "127.0.0.1:9996";
    let server = Server::new();
    let mut handler = server.clone();
    thread::spawn(move || {
        handler.listen(ip_addr); // NOTE: this fails silently if server is already running
    });

    // load in client and connect to server
    let client;
    let rx: Receiver<Event>;
    if let Some((mut c, rx_)) = Client::load_file("client.key") { //assumes client key is an admin user
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
    
    
    let mut ifc = Interface::init("stratis server", [800,600]);
    let mut app = AppState::default();
    
    'main: loop {
        if !ifc.render(None, |ui| {
            app.render(&ui,
                       &*client.lock().expect("Client mutex poisoned"));
        }) { break 'main }

        ifc.maybe_sleep()
    }
}

struct AppState {
    player_idx: i32,
}

impl Default for AppState {
    fn default() -> AppState {
        AppState {
            player_idx: -1,
        }
    }
}

impl AppState {
    fn render(&mut self, ui: &Ui, client: &Client) {
        ui.window(im_str!("Players"))
            .build(|| {
                let players: Vec<ImString> = 
                    client.player_cache.iter().map(|p| {
                        im_str!("{:}", p.0).to_owned()
                    }).collect();

                let players: Vec<&ImStr> = players.iter().map(|p|p.as_ref()).collect();

                ui.list_box(im_str!("UIDS"),
                            &mut self.player_idx,
                            &players[..],
                            50i32);
            });
    }
}
