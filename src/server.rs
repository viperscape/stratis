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
extern crate rand;

use self::uuid::Uuid;


use client::{Client};
use chat::{read_text,text_as_bytes};
use distributor::Distributor;
use distributor::Kind as DistKind;
use store::{DataStore,Store};
use player::Player;

pub struct Server {
    clients: Vec<Client>, // this always grows
    players: HashMap<Uuid, Player>,
    pub dist_tx: Sender<DistKind<TcpStream>>,
    store: Store,
}

impl Server {
    pub fn new(ip: &str) {
        let listener = TcpListener::bind(ip).unwrap();
        
        let (dist_tx,mut dist) = Distributor::new(Store::default());
        thread::spawn(move || dist.run());
        
        let mut server = Server {
            clients: vec!(),
            players: HashMap::new(),
            dist_tx: dist_tx,
            store: Store::default(),
        };

        server.clients = server.store.clients_get();
        
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
                        Server::register(&mut server, &mut s);
                    },
                    _ => {
                        if client_idx.is_some() {
                            match cmd[0] {
                                2 => { //chat
                                    Server::chat(&mut server, &mut s,
                                                 client_idx.unwrap());
                                },
                                3 => { //nick
                                    Server::nick(&mut server, &mut s,
                                                 client_idx.unwrap());
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

    fn register (server: &mut Arc<Mutex<Server>>,
                 mut s: &mut TcpStream,) {
        if let Some(c) = Client::load(&mut s) {
            let mut server = server.lock().unwrap();
            
            for n in server.clients.iter() {
                if n.id == c.id { continue }
            }

            let r = server.store.client_put(&c);
            println!("registered ({:?}):{:?}",r, c.id);

            let mut nick = "player_".to_string();
            nick.push_str(&rand::random::<u16>().to_string());
            println!("nick:{:?}",nick);
            server.store.player_put(&c.id,nick);
            
            
            server.clients.push(c);
        }
    }

    #[allow(unused_must_use)]
    fn chat (server: &mut Arc<Mutex<Server>>,
             mut s: &mut TcpStream,
             client_idx: usize) {
        
        if let Some(text) = read_text(s) {
            println!("chat-client:{:?}",text.trim());
            
            //broadcast
            let (mut data, bytes) = text_as_bytes(&text);
            data.extend_from_slice(bytes);

            
            let server = server.lock().unwrap();
            let uuid = server.clients[client_idx].id;
            data.extend_from_slice(uuid.as_bytes()); //refer to uuid
            
            server.dist_tx.send(DistKind::Broadcast(data));
        }
    }

    #[allow(unused_must_use)]
    fn nick (server: &mut Arc<Mutex<Server>>,
             mut s: &mut TcpStream,
             client_idx: usize) {
        
        if let Some(text) = read_text(s) {            
            //broadcast
            let (mut data, bytes) = text_as_bytes(&text);
            data[0] = 3; //change opcode to nick
            data.extend_from_slice(bytes);
            
                
            let server = server.lock().unwrap();
            let uuid = server.clients[client_idx].id;
            data.extend_from_slice(uuid.as_bytes()); //refer to uuid
            
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
                    reg_key = Some(n.key.clone());
                    client_idx = Some(i);
                    
                    break
                }
            }

            if let Some(key) = reg_key {
                let hm = hmacsha1::hmac_sha1(&key, m.as_bytes());
                if c.key == hm {
                    let mut nick = None;
                    
                    if let Some(n) = server.store.player_get(&c.id) {
                        nick = Some(n);
                    }
                    
                    if let Some(nick) = nick {
                        server.players.insert(c.id,
                                              Player { client_idx:client_idx.unwrap(),
                                                       nick: nick });
                    }
                    else { panic!("{:?} missing nick", c.id) }
                    

                    if let Ok(stmp) = s.try_clone() {
                        server.dist_tx.send(DistKind::Add(c.id,stmp));
                    }
                    
                    println!("login:{:?}",c.id);
                    println!("total clients:{:?}",server.players.len());
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
