extern crate stratis_shared as shared;
extern crate stratis;

use stratis::{Server,Client};
use shared::opcode;
use shared::events::Event;

use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;

fn assert_opcode (rx: &Receiver<Event>, op: u8) -> Event {
    match rx.try_recv() {
        Ok(ev) => {
            assert_eq!(ev.0,op);
            return ev
        },
        _ => { panic!("no events received"); }
    }
}

fn assert_client () -> (Arc<Mutex<Client>>,Receiver<Event>) {
    let host = "127.0.0.1:65432";
    let _handle = thread::spawn(move || {
        let mut _server = Server::new(host);
    });
    let (mut client, rx) = Client::default();
    client.connect(host);
    client.register();

    let client = Arc::new(Mutex::new(client));
    Client::login(&client);

    assert!(client.lock().unwrap().stream.is_some());

    (client,rx)
}

#[test]
fn client () {
    let (_client, rx) = assert_client();
    
    thread::sleep(Duration::from_millis(100));

    // we recv player twice
    // once for our own login
    // and again for each player that's connected
    assert_opcode(&rx,opcode::PLAYER);
    assert_opcode(&rx,opcode::PLAYER);
    
    thread::sleep(Duration::from_millis(100));
}
