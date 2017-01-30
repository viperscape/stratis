use std::io;
use std::thread;

mod game;
mod client;
mod server;
mod distributor;
mod chat;
mod store;

use client::Client;

#[allow(unused_must_use)]
fn main() {
    let mut input = String::new();
    
    let _game = game::Game::new();
    let ip_addr = "127.0.0.1:9996";
    
    let _server_thread = thread::spawn(move || {
        server::Server::new(ip_addr);
    });
    
    let mut client;
    if let Some(c) = Client::load_file("game/client.key") {
        client = c;
        client.connect(ip_addr);
        client.login();
    }
    else {
        client = Client::new("game/client.key");
        client.connect(ip_addr);
        client.register();
        client.login();
    }

    
    let mut chat = true;
    loop {
        input.clear();
        
        if let Ok(_) = io::stdin().read_line(&mut input) {
            let cmd = input.trim();
            match cmd {
                "comm" => { chat = !chat; println!("comm online:{:?}",chat); }
                _ => {
                    if chat { client.chat(&input) }
                    else {
                        match cmd {
                            "exit" => { break },
                            _ => { }
                        }
                    }
                },
            }            
        }
    }
}
