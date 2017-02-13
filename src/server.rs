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


use client::{Client,ClientBase};
use chat::{read_text,text_as_bytes};
use distributor::Distributor;
use distributor::Kind as DistKind;
use store::{DataStore,Store};
use player::Player;

pub struct Server {
    clients: Vec<ClientBase>, // this always grows
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

        let mut client_id: Option<Uuid> = None;
        
        loop {
            if let Ok(_) = s.read_exact(&mut cmd) {
                match cmd[0] {
                    0 => { //login
                        client_id = Server::login(&mut server, &mut s, m);
                        if let Some(uuid) = client_id {
                            Server::send_nicks(&mut server, uuid);
                        }
                    },
                    1 => { //register
                        Server::register(&mut server, &mut s);
                    },
                    _ => {
                        if let Some(uuid) = client_id {
                            match cmd[0] {
                                2 => { //chat
                                    Server::chat(&mut server, &mut s, uuid);
                                },
                                3 => { //nick
                                    Server::nick(&mut server, &mut s, uuid);
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
                if &n.id == c.id() { continue }
            }

            let mut r = false;
            for _ in [..100].iter() { // try to repeatedly add a unique name
                let mut nick = "player_".to_string();
                nick.push_str(&rand::random::<u16>().to_string());
                r = server.store.player_put(&c.id(), &Player { nick: nick });
                if r { break }
            }

            if r {
                let r = server.store.client_put(&c.base);
                println!("registered ({:?}):{:?}",r, c.id());
                server.clients.push(c.base);
            }
        }
    }

    #[allow(unused_must_use)]
    fn chat (server: &mut Arc<Mutex<Server>>,
             mut s: &mut TcpStream,
             uuid: Uuid) {
        
        if let Some(text) = read_text(s) {
            println!("chat-client:{:?}",text.trim());
            
            //broadcast
            let (mut data, bytes) = text_as_bytes(&text);
            data.extend_from_slice(bytes);

            
            let server = server.lock().unwrap();
            data.extend_from_slice(uuid.as_bytes()); //refer to uuid
            
            server.dist_tx.send(DistKind::Broadcast(data));
        }
    }

    #[allow(unused_must_use)]
    fn nick (server: &mut Arc<Mutex<Server>>,
             mut s: &mut TcpStream,
             uuid: Uuid) {
        
        if let Some(text) = read_text(s) {
            let r = {
                let server = server.lock().unwrap();
                server.store.player_update(&uuid, &Player { nick: text.clone() })
            };

            if r {
                Server::send_nick(server, uuid,&text);
                println!("nick_change:{:?}  {:?}",uuid,text);
            }
        }
    }
     #[allow(unused_must_use)]
    fn send_nick (server: &mut Arc<Mutex<Server>>,
                  uuid: Uuid,
                  nick: &String) {
        //broadcast
        let (mut data, bytes) = text_as_bytes(nick);
        data[0] = 3; //change opcode to nick
        data.extend_from_slice(bytes);
        data.extend_from_slice(uuid.as_bytes()); //refer to uuid
        
        let server = server.lock().unwrap();
        server.dist_tx.send(DistKind::Broadcast(data));
    }

    #[allow(unused_must_use)]
    fn login (server: &mut Arc<Mutex<Server>>,
              mut s: &mut TcpStream,
              m: Uuid) -> Option<Uuid> {
        let mut client_id = None;
        let mut player = None;
        
        if let Some(c) = Client::load(&mut s) {
            let mut reg_key = None;
            
            {
                let mut server = server.lock().unwrap();
                for n in server.clients.iter_mut() {
                    if &n.id == c.id() {
                        reg_key = Some(n.key.clone());
                        client_id = Some(c.id().clone());
                        
                        break
                    }
                }
            }

            if let Some(key) = reg_key {
                let hm = hmacsha1::hmac_sha1(&key, m.as_bytes());
                if c.key() == hm {
                    {
                        let mut server = server.lock().unwrap();
                        player = server.store.player_get(c.id());
                    }
                    
                    if let Some(player) = player {
                        if let Ok(stmp) = s.try_clone() {
                            {
                                let mut server = server.lock().unwrap();
                                server.dist_tx.send(DistKind::Add(*c.id(),stmp));
                                server.players.insert(*c.id(),player.clone());
                                println!("total players:{:?}",server.players.len());
                            }

                            println!("login:{:?}",c.id());
                            Server::send_nick(server, *c.id(), &player.nick);
                        }
                    }
                }
                else {
                    panic!("client invalid login {:?}", c)
                }

            }
            else { panic!("client unregistered {:?}", c) }
        }

       client_id
    }

    #[allow(unused_must_use)]
    //NOTE: this is very innefficient, and hogs the mutex
    //TODO: rework state updates entirely
    fn send_nicks (server: &mut Arc<Mutex<Server>>,
                   uuid: Uuid)  {
        let server = server.lock().unwrap();
        
        for (ref pid,player) in server.players.iter() {
            let (mut data, bytes) = text_as_bytes(&player.nick);
            data[0] = 3; // specify nick route
            
            data.extend_from_slice(bytes);
            data.extend_from_slice(pid.as_bytes()); //refer to uuid
            
            
            server.dist_tx.send(DistKind::Select(uuid,data));
        }
    }
}
