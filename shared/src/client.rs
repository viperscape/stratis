extern crate hmacsha1;
extern crate uuid;
extern crate byteorder;

use self::uuid::Uuid;

use std::time::{Instant,Duration};
use std::io::prelude::*;
use std::net::Shutdown;
use std::fs::File;
use std::net::TcpStream;
use std::collections::HashMap;

use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel,Receiver,Sender};

use player::Player;
use chat::{read_text,write_text};
use opcode;
use events::Event;

pub const KEY_LEN: usize = 20;
pub const ID_LEN: usize = 16;

#[derive(Debug,Clone)]
pub struct ClientBase {
    pub key: Vec<u8>,
    pub id: Uuid,
}

#[derive(Debug)]
pub struct Client {
    pub base: ClientBase,
    pub stream: Option<TcpStream>,
    
    pub player_cache: HashMap<Uuid,Player>,
    pub msg_cache: HashMap<Uuid,Vec<String>>, //NOTE: 'events' orders of these messages
    
    pub ping_start: Instant,
    pub ping_delta: f32,

    pub events: Sender<Event>,
}

impl Client {
    #[allow(dead_code)]
    pub fn new (key: [u8;KEY_LEN], uuid: Uuid) -> (Client, Receiver<Event>) {
        let (tx,rx) = channel();
        
        (Client { base: ClientBase { key:From::from(&key[..]),
                                     id:uuid, },
                  stream: None,
                  player_cache: HashMap::new(),
                  msg_cache: HashMap::new(),
                  
                  ping_start: Instant::now(),
                  ping_delta: 0.0,

                  events: tx,
        },
         rx)
    }
    
    #[allow(unused_must_use)]
    pub fn default () -> (Client, Receiver<Event>) {        
        let id = uuid::Uuid::new_v4();
        let m = hmacsha1::hmac_sha1(uuid::Uuid::new_v4().as_bytes(),
                                    id.as_bytes());

       
        Client::new(m,id)
    }
    #[allow(unused_must_use)]
    pub fn save (client: &Client, path: &str) -> bool {
        let f = File::create(path);
        if let Ok(mut f) = f {
            f.write_all(&client.base.key);
            f.write_all(client.base.id.as_bytes()).is_ok()
        }
        else { false }
    }

    pub fn id (&self) -> &Uuid {
        &self.base.id
    }

    pub fn key (&self) -> &[u8] {
        &self.base.key[..]
    }

    #[allow(dead_code)]
    pub fn key_as_ref (&self) -> &Vec<u8> {
        &self.base.key
    }

    pub fn load_file (path: &str) -> Option<(Client, Receiver<Event>)> {
        let f = File::open(path);
        if let Ok(mut f) = f {
            return Client::load(&mut f)
        }

        None
    }

    pub fn load<S:Read> (s: &mut S) -> Option<(Client, Receiver<Event>)> {
        let mut key = [0u8;KEY_LEN];
        let mut id = [0u8;ID_LEN];
        if let Ok(_) = s.read_exact(&mut key) {
            if let Ok(_) = s.read_exact(&mut id) {
                if let Ok(id) = Uuid::from_bytes(&id) {
                    return Some(Client::new(key, id))
                }
                else { println!("cannot uuid file") }
            }
        }

        None
    }

    #[allow(unused_must_use)]
    pub fn connect (&mut self, server: &str)  {
        if let Ok(s) = TcpStream::connect(server) {
            self.stream = Some(s);
        }
    }

    #[allow(unused_must_use)]
    pub fn login (client: &Arc<Mutex<Client>>) -> bool {
        let c = client.clone();
        if let Ok(ref mut client) = client.lock() {
            let base = client.base.clone();
            
            if let Some(ref mut s) = client.stream {
                
                let mut m = [0u8;ID_LEN];
                if let Ok(_) = s.read_exact(&mut m) {
                    s.write_all(&[0]);
                    
                    let hm = hmacsha1::hmac_sha1(&base.key, &m);
                    
                    s.write_all(&hm);
                    s.write_all(base.id.as_bytes());

                    
                    thread::spawn(move || {
                        Client::handler(c)
                    });

                    return true
                }
            }
        }

        false
    }

    #[allow(unused_must_use)]
    pub fn register (&mut self) {
        if let Some(ref mut s) = self.stream {
            s.write_all(&[1]);
            s.write_all(&self.base.key);
            s.write_all(self.base.id.as_bytes());
        }
    }

    #[allow(unused_must_use)]
    pub fn chat (&mut self, text: &str) {
        if let Some(ref mut s) = self.stream {
            write_text(s, text);
        }
    }

    #[allow(unused_must_use)]
    pub fn nick (&mut self, nick: &str) {
        if let Some(ref mut s) = self.stream {
            s.write_all(&Player::to_bytes(None,
                                          &Player { nick: nick.to_owned() }));
        }
    }

    #[allow(unused_must_use)]
    pub fn ping (&mut self) -> bool {
        if let Some(ref mut s) = self.stream {
            self.ping_start = Instant::now();
            
            return s.write(&[opcode::PING]).is_ok()
        }

        false
    }

    
    pub fn handler (client: Arc<Mutex<Client>>) {
        let mut cmd = [0u8;1];
        let mut s; // NOTE: this should be read from only!

        if let Ok(mut client) = client.lock() {
            client.ping();
            
            if let Some(ref s_) = client.stream {
                if let Ok(s_) = s_.try_clone() {
                    s = s_;
                }
                else { return }
            }
            else { return }
        }
        else { return }
        
        'handler: loop {
            if let Ok(_) = s.read_exact(&mut cmd) {
                match cmd[0] {
                    opcode::CHAT => {
                        if let Some(text) = read_text(&mut s) {
                            let mut id = [0u8;ID_LEN];
                            if let Ok(_) = s.read_exact(&mut id) {
                                if let Ok(uuid) = Uuid::from_bytes(&id) {
                                    if let Ok(mut client) = client.lock() {

                                        // this lil dance makes BC happy
                                        let mut has_uuid = false;
                                        if let Some(msg) = client.msg_cache.get_mut(&uuid) {
                                            msg.push(text.clone());
                                            has_uuid = true;
                                        }
                                        
                                        if !has_uuid { client.msg_cache.insert(uuid,vec![text]); }
                                        
                                        let _ = client.events.send(Event(opcode::CHAT,
                                                                           uuid));
                                    }
                                }
                            }
                        }
                    },
                    opcode::PLAYER => {
                        if let (Some(uuid),Some(player)) = Player::from_stream(&mut s, true) {
                            if let Ok(mut client) = client.lock() {
                                client.player_cache.insert(uuid, player);
                                
                                let _ = client.events.send(Event(opcode::PLAYER,
                                                                   uuid));
                            }
                        }
                    },
                    opcode::PING => { // recv server ping, respond
                        if let Ok(mut client) = client.lock() {
                            if let Some(ref mut s) = client.stream {
                                let _ = s.write_all(&[opcode::PONG]);
                            }
                        }
                    },
                    opcode::PONG => { // recv client ping back?
                        if let Ok(mut client) = client.lock() {
                            client.ping_delta = client.ping_start.elapsed().as_secs() as f32;
                        }
                    },
                    _ => { },
                }
            }

            // check ping delay, shutdown
            if let Ok(mut client) = client.lock() {
                client.ping_delta += client.ping_start.elapsed().as_secs() as f32;
                if client.ping_delta > 8.0 { break 'handler }
                
                // ping every so often
                if client.ping_start.elapsed() > Duration::new(5, 0) {
                    if !client.ping() { break 'handler }
                }
            }
        }
        
        if let Ok(mut client) = client.lock() {
            client.shutdown();
        }
    }

    pub fn shutdown(&mut self) {
        if let Some(ref mut s) = self.stream {
            let _ = s.flush();
            let _ = s.shutdown(Shutdown::Both);
        }

        self.stream = None;
    }
}

impl Drop for Client {
    fn drop (&mut self) {
        self.shutdown();
    }
}
