use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;

use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender};

use std::io::prelude::*;

extern crate hmacsha1;
extern crate uuid;
extern crate byteorder;

use shared::{chat,client,opcode,player};
use self::uuid::Uuid;


use self::client::{Client,ClientBase};
use self::chat::{read_text,text_as_bytes};
use distributor::Distributor;
use distributor::Kind as DistKind;
use store::{DataStore,Store};
use self::player::Player;

#[derive(Debug,Clone)]
pub struct Server {
    clients: Arc<Mutex<Vec<ClientBase>>>, // this always grows
    players: Arc<Mutex<HashMap<Uuid, Player>>>, //NOTE: this may make more sense as atomic-vec-keys
    dist: Sender<DistKind<TcpStream>>,
    store: Arc<Mutex<Store>>, //NOTE: this may become a threadpool
}

impl Server {
    pub fn new(ip: &str) {
        let listener = TcpListener::bind(ip).unwrap();
        
        let (dist_tx, mut distributor) = Distributor::new(Store::default());
        thread::spawn(move || distributor.run());
        
        let server = Server {
            clients: Arc::new(Mutex::new(vec!())),
            players: Arc::new(Mutex::new(HashMap::new())),
            dist: dist_tx,
            store: Arc::new(Mutex::new(Store::default())),
        };

        if let Ok(mut clients) = server.clients.lock() {
            if let Ok(store) = server.store.lock() {
                *clients = store.clients_get();
            }
        }
        
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
    fn handler (mut server: Server, mut s: TcpStream) {
        let mut cmd = [0;1];
                
        //new conn needs auth code
        let m = uuid::Uuid::new_v4();
        s.write_all(m.as_bytes());

        let mut client_id: Option<Uuid> = None;
        
        loop {
            if let Ok(_) = s.read_exact(&mut cmd) {
                match cmd[0] {
                    opcode::LOGIN => { //login
                        client_id = Server::login(&mut server, &mut s, m);
                        if let Some(uuid) = client_id {
                            Server::send_players(&mut server, uuid);
                        }
                    },
                    opcode::REGISTER => { //register
                        Server::register(&mut server, &mut s);
                    },
                    _ => {
                        if let Some(uuid) = client_id {
                            match cmd[0] {
                                opcode::CHAT => { //chat
                                    Server::handle_chat(&mut server, &mut s, uuid);
                                },
                                opcode::PLAYER => { //player
                                    if let (_, Some(player)) = Player::from_stream(&mut s, false) {
                                        if let Ok(store) = server.store.lock() {
                                            if store.player_update(&uuid, &player) {
                                                let data = Player::to_bytes(Some(&uuid),&player);
                                                server.dist.send(DistKind::Broadcast(data));
                                                
                                                println!("nick_change:{:?}  {:?}",uuid,player);
                                            }
                                        }
                                    }
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

    fn register (server: &mut Server,
                 mut s: &mut TcpStream,) {
        if let Some(c) = Client::load(&mut s) {

            if let Ok(clients) = server.clients.lock() {
                for n in clients.iter() {
                    if &n.id == c.id() { panic!("client already register {:?}",c.id()) }
                }
            }

            if let Ok(store) = server.store.lock() {
                let mut r = false;
                for _ in [..100].iter() { // try to repeatedly add a unique name
                    r = store.player_put(&c.id(), &Player::default());
                    if r { break }
                }

                if r {
                    let r = store.client_put(&c.base);
                    println!("registered ({:?}):{:?}",r, c.id());

                    if let Ok(mut clients) = server.clients.lock() {
                        clients.push(c.base);
                    }
                }

            }
        }
    }

    #[allow(unused_must_use)]
    fn handle_chat (server: &mut Server,
             mut s: &mut TcpStream,
             uuid: Uuid) {
        
        if let Some(text) = read_text(s) {
            println!("chat-client:{:?}",text.trim());
            
            //broadcast
            let (mut data, bytes) = text_as_bytes(&text);
            data.extend_from_slice(bytes);
            data.extend_from_slice(uuid.as_bytes()); //refer to uuid
            
            server.dist.send(DistKind::Broadcast(data));
        }
    }

    #[allow(unused_must_use)]
    fn login (server: &mut Server,
              mut s: &mut TcpStream,
              m: Uuid) -> Option<Uuid> {
        let mut client_id = None;
        let mut player = None;
        
        if let Some(c) = Client::load(&mut s) {
            let mut reg_key = None;
            
            if let Ok(clients) = server.clients.lock() {
                for n in clients.iter() {
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
                    
                    if let Ok(store) = server.store.lock() {
                        player = store.player_get(c.id());
                    }
                    
                    if let Some(player) = player {
                        if let Ok(stmp) = s.try_clone() {
                            {
                                server.dist.send(DistKind::Add(*c.id(),stmp));

                                if let Ok(mut players) = server.players.lock() {
                                    players.insert(*c.id(),player.clone());
                                    println!("total players:{:?}",players.len()); //NOTE: this should print on debug only
                                }
                            }

                            println!("login:{:?}",c.id());
                            let data = Player::to_bytes(Some(c.id()),&player);
                            server.dist.send(DistKind::Broadcast(data));
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
    fn send_players (server: &mut Server,
                     uuid: Uuid)  {

        if let Ok(players) = server.players.lock() {
            for (ref pid,player) in players.iter() {
                let data = Player::to_bytes(Some(pid),&player);
                
                server.dist.send(DistKind::Select(uuid,data));
            }

        }
    }
}