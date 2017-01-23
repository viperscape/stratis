use std::io;


mod game;
mod client;

#[allow(unused_must_use)]
fn main() {
    let mut input = String::new();
    
    let game = game::Game::new();
    let client = client::Client::load("game/client.key").unwrap_or
        (client::Client::new("game/client.key"));
    
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
}
