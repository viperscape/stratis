use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;

use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};

use std::io::prelude::*;
use std::io::BufReader;

extern crate hmacsha1;
extern crate uuid;

use self::uuid::Uuid;
use client::Client;
use distributor::Distributor;

pub struct Player {
    client_idx: usize, //this is dynamic in the sense that it may be different on intial run
}



pub struct Server {
    clients: Vec<Client>, // this always grows
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

        let mut client_idx = None;
        
        loop {
            if let Ok(_) = s.read_exact(&mut cmd) {
                match cmd[0] {
                    0 => { //login
                        if let Some(c) = Client::load(&mut s) {
                            let mut server = server.lock().unwrap();

                            let mut reg_key = None;
                            for (i,n) in server.clients.iter_mut().enumerate() {
                                if n.id == c.id {
                                    reg_key = Some(n.key);
                                    client_idx = Some(i);
                                    if let Ok(stmp) = s.try_clone() {
                                        n.stream = Some(Arc::new(Mutex::new(stmp)));
                                    }
                                    break
                                }
                            }

                            if let Some(key) = reg_key {
                                let hm = hmacsha1::hmac_sha1(&key, m.as_bytes());
                                if c.key == hm {
                                    server.players.insert(c.id,
                                                          Player {client_idx:client_idx.unwrap()});
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
                    2 => { //chat
                        let mut text = String::new();
                        {
                            let mut bs = BufReader::new(&s);
                            bs.read_line(&mut text);
                        }

                        if text.chars().count() > 0 {
                            println!("chat-client:{:?}",text.trim());

                            //echo back
                            s.write_all(&[2]);
                            s.write_all(&text.as_bytes());
                        }
                    },
                    _ => panic!("cmd:{:?}",cmd)
                }
            }
        }

        println!("client dropped");
    }
}
