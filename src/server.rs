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
        loop {
            if let Ok(_) = s.read_exact(&mut cmd) {
                match cmd[0] {
                    0 => { //login
                        if let Some(c) = Client::load(&mut s) {
                            let mut server = server.lock().unwrap();

                            let mut is_reg = false;
                            for n in server.clients.iter() {
                                if n.id == c.id { is_reg = true }
                            }

                            if is_reg {
                                let mut m = [0u8;16];
                                if let Ok(_) = s.read_exact(&mut m) { //read in login key
                                    let hm = hmacsha1::hmac_sha1(&c.key, &m);
                                    
                                    server.players.insert(c.id,Player);
                                    println!("login:{:?}",c.id);
                                    println!("total clients:{:?}",server.clients.len());
                                }
                            }
                            else { panic!("client unregistered {:?}", c.id) }
                        }
                    },
                    1 => { //register
                        if let Some(c) = Client::load(&mut s) {
                            let mut server = server.lock().unwrap();
                            
                            let mut is_reg = false;
                            for n in server.clients.iter() {
                                if n.id == c.id { is_reg = true }
                            }

                            if !is_reg {
                                println!("registered:{:?}",c.id);
                                server.clients.push(c);
                            }
                        }
                    },
                    _ => panic!("cmd:{:?}",cmd)
                }
            }
        }

        println!("client dropped");
    }
}
