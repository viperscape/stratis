extern crate stratis_shared as shared;

use shared::client::{Client};
use shared::events::Event;

use std::io;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;


#[allow(unused_must_use)]
fn main() {
    let mut input = String::new();
    
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
    
    let mut chat = true;
    loop {
        input.clear();
        let mut client = client.lock().unwrap();
        
        if let Ok(_) = io::stdin().read_line(&mut input) {
            let cmd = input.trim();
            match cmd {
                "comm" => { chat = !chat; println!("comm online:{:?}",chat); }
                _ => {
                    if chat { client.chat(&input) }
                    else {
                        match cmd {
                            "exit" => { break },
                            "ev" => {
                                if let Ok(e) = rx.try_recv() {
                                    println!("event:{:?}",e);
                                }
                            },
                            _ => {
                                let cmds: Vec<&str> = cmd.split(' ').collect();
                                if cmds[0] == "nick" {
                                    println!("nick:{:?}",cmds[1]);
                                    client.nick(cmds[1]);
                                }
                            }
                        }
                    }
                },
            }            
        }
    }
}
