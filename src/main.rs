use std::io;
use std::thread;

mod game;
mod client;
mod server;

#[allow(unused_must_use)]
fn main() {
    let mut input = String::new();
    
    let game = game::Game::new();
    let ip_addr = "127.0.0.1:9996";
    
    let serve = thread::spawn(move || {
        server::Server::new(ip_addr);
    });
    
    let mut client;
    if let Some(c) = client::Client::load_file("game/client.key") {
        client = c;
        client.connect(ip_addr);
        client.register(); // debug: register anyways
        client.login();
    }
    else {
        client = client::Client::new("game/client.key");
        client.connect(ip_addr);
        client.register();
        client.login();
    }
    
    
    loop {
        input.clear();
        break; //debug break
        
        if let Ok(_) = io::stdin().read_line(&mut input) {
            println!("{:?}",input.trim());

            match &input.trim() {
                &"exit" => { break },
                _ => { },
            }
           
        }
    }

    serve.join();
}
