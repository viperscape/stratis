use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;

use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender};

use std::io::prelude::*;
//use std::io::BufReader;

extern crate hmacsha1;
extern crate uuid;
extern crate byteorder;

use self::uuid::Uuid;


use client::{Client};
use chat::{read_text,text_as_bytes};
use distributor::Distributor;
use distributor::Kind as DistKind;
use store::Store;

#[allow(dead_code)]
pub struct Player {
    client_idx: usize, //this is dynamic in the sense that it may be different on intial run
}



pub struct Server {
    clients: Vec<Client>, // this always grows
    players: HashMap<Uuid, Player>,
    pub dist_tx: Sender<DistKind<TcpStream>>,
    store: Option<Store>,
}

impl Server {
    pub fn new(ip: &str) {
        let listener = TcpListener::bind(ip).unwrap();
        
        let (dist_tx,mut dist) = Distributor::new();
        thread::spawn(move || dist.run());
        
        let server = Server {
            clients: vec!(),
            players: HashMap::new(),
            dist_tx: dist_tx,
            store: Store::new(),
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

    #[allow(unused_must_use)]
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
                        client_idx = Server::login(&mut server, &mut s, m);
                    },
                    1 => { //register
                        if let Some(c) = Client::load(&mut s) {
                            let mut server = server.lock().unwrap();
                            
                            for n in server.clients.iter() {
                                if n.id == c.id { continue }
                            }

                            let mut key = vec!();
                            key.extend_from_slice(&c.key); //FIXME: postgres execute doesn't like &[u8] for ToSql traits
                            if let Some(ref store) = server.store {
                                store.conn.execute("INSERT INTO clients (uuid, key) VALUES ($1, $2)",
                                                   &[&c.id, &key]);
                            }
                            
                            println!("registered:{:?}",c.id);
                            server.clients.push(c);
                        }
                    },
                    _ => {
                        if client_idx.is_some() {
                            match cmd[0] {
                                2 => { //chat
                                    Server::chat(&mut server, &mut s);
                                },
                                _ => panic!("unknown cmd:{:?}",cmd)
                            }
                        }
                    }
                }
            }
            else { break } //drop dead client
        }

        println!("client dropped");
    }

    #[allow(unused_must_use)]
    fn chat (server: &mut Arc<Mutex<Server>>,
             mut s: &mut TcpStream,) {
        
        if let Some(text) = read_text(s) {
            println!("chat-client:{:?}",text.trim());
            
            //broadcast
            let (mut data, bytes) = text_as_bytes(&text);
            data.extend_from_slice(bytes);
                
            let server = server.lock().unwrap();
            server.dist_tx.send(DistKind::Broadcast(data));
        }
    }

    #[allow(unused_must_use)]
    fn login (server: &mut Arc<Mutex<Server>>,
              mut s: &mut TcpStream,
              m: Uuid) -> Option<usize> {
        let mut client_idx = None;
        
        if let Some(c) = Client::load(&mut s) {
            let mut server = server.lock().unwrap();

            let mut reg_key = None;
            for (i,n) in server.clients.iter_mut().enumerate() {
                if n.id == c.id {
                    reg_key = Some(n.key);
                    client_idx = Some(i);
                    
                    break
                }
            }

            if let Some(key) = reg_key {
                let hm = hmacsha1::hmac_sha1(&key, m.as_bytes());
                if c.key == hm {
                    server.players.insert(c.id,
                                          Player {client_idx:client_idx.unwrap()});

                    if let Ok(stmp) = s.try_clone() {
                        //n.stream = Some(Arc::new(Mutex::new(stmp)));
                        server.dist_tx.send(DistKind::Add(c.id,stmp));
                    }
                    
                    println!("login:{:?}",c.id);
                    println!("total clients:{:?}",server.clients.len());
                }
                else {
                    panic!("client invalid login {:?}", c)
                }

            }
            else { panic!("client unregistered {:?}", c) }
        }

       client_idx
    }
}
