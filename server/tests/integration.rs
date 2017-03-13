extern crate stratis_shared as shared;
extern crate stratis;

extern crate rand;

use stratis::{Server,Client};
use shared::opcode;
use shared::events::Event;

use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;

use rand::distributions::{IndependentSample,Range};

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
    let btw = Range::new(25000,64000);
    let mut rng = rand::thread_rng();

    let v = btw.ind_sample(&mut rng);
    println!("{:?}",v);
    
    let host = "127.0.0.1:".to_owned() + stringify!(btw.ind_sample(&mut rng));
    let host_ = host.clone();
    let _handle = thread::spawn(move || {
        let mut _server = Server::new(&host_);
    });
    let (mut client, rx) = Client::default();
    client.connect(&host);
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
}

#[test]
fn client_chat () {
    let (client, rx) = assert_client();
    
    thread::sleep(Duration::from_millis(100));
    let text = "test";
    client.lock().unwrap().chat(text);
    
    assert_opcode(&rx,opcode::PLAYER);
    assert_opcode(&rx,opcode::PLAYER);
    let id = assert_opcode(&rx,opcode::CHAT).1;
    let text_ = client.lock().unwrap()
        .msg_cache.get_mut(&id).unwrap()
        .remove(0);

    assert_eq!(text,text_);
}
