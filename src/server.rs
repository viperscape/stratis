use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;

use std::thread;
use std::sync::{Arc, Mutex};

use std::io::prelude::*;

extern crate hmacsha1;
extern crate uuid;

use self::uuid::Uuid;
use client::Client;

pub struct Player;

pub struct Server {
    clients: Vec<Client>,
    players: HashMap<Uuid, Player>,
}

impl Server {
    pub fn new(ip: &str) {
        let listener = TcpListener::bind(ip).unwrap();

        let server = Server {
            clients: vec!(),
            players: HashMap::new(),
        };
        let server = Arc::new(Mutex::new(server));
        
        for s in listener.incoming() {
            match s {
                Ok(s) => {
                    let server = server.clone();
                    thread::spawn(|| Server::handler(server,s));
                },
                _ => {},
            }
        }
    }

    fn handler (mut server: Arc<Mutex<Server>>, mut s: TcpStream) {
        let mut cmd = [0;1];

        //new conn needs auth code
        let m = uuid::Uuid::new_v4();
        s.write_all(m.as_bytes());
        
        loop {
            if let Ok(_) = s.read_exact(&mut cmd) {
                match cmd[0] {
                    0 => { //login
                        if let Some(c) = Client::load(&mut s) {
                            let mut server = server.lock().unwrap();

                            let mut reg_key = None;
                            for n in server.clients.iter() {
                                if n.id == c.id { reg_key = Some(n.key) }
                            }

                            if let Some(key) = reg_key {
                                let hm = hmacsha1::hmac_sha1(&key, m.as_bytes());
                                if c.key == hm {
                                    server.players.insert(c.id,Player);
                                    println!("login:{:?}",c.id);
                                    println!("total clients:{:?}",server.clients.len());
                                }
                                else {
                                    panic!("client invalid login {:?}", c)
                                }

                            }
                            else { panic!("client unregistered {:?}", c) }
                        }
                    },
                    1 => { //register
                        if let Some(c) = Client::load(&mut s) {
                            let mut server = server.lock().unwrap();
                            
                            for n in server.clients.iter() {
                                if n.id == c.id { continue }
                            }

                            println!("registered:{:?}",c.id);
                            server.clients.push(c);
                        }
                    },
                    _ => panic!("cmd:{:?}",cmd)
                }
            }
        }

        println!("client dropped");
    }
}
