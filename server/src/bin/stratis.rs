extern crate stratis;

use stratis::{Client,Server};

use std::io;
use std::thread;
use std::sync::{Arc, Mutex};


#[allow(unused_must_use)]
fn main() {
    let mut input = String::new();
    
    let ip_addr = "127.0.0.1:9996";
    
    //let _server_thread = thread::spawn(move || {
        Server::new(ip_addr);
    //});

    /*
    let client;
    if let Some(mut c) = Client::load_file("game/client.key") {
        c.connect(ip_addr);
        client = Arc::new(Mutex::new(c));
        Client::login(&client);
    }
    else {
        let mut c = Client::default();
        Client::save(&c,"game/client.key");
        c.connect(ip_addr);
        c.register();
        client = Arc::new(Mutex::new(c));
        Client::login(&client);
    }
    
    
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
     */
}
