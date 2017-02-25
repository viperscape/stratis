extern crate hmacsha1;
extern crate uuid;
extern crate byteorder;

use self::uuid::Uuid;

use std::io::prelude::*;
use std::net::Shutdown;
use std::fs::File;
use std::net::TcpStream;
use std::collections::HashMap;

use std::thread;
use std::sync::{Arc, Mutex};

use player::Player;
use chat::{read_text,write_text, text_as_bytes};
use opcode;

pub const KEY_LEN: usize = 20;
pub const ID_LEN: usize = 16;

#[derive(Debug,Clone)]
pub struct ClientBase {
    pub key: Vec<u8>,
    pub id: Uuid,
}

#[derive(Debug,Clone)]
pub struct Client {
    pub base: ClientBase,
    pub stream: Option<Arc<Mutex<TcpStream>>>,
    pub cache: HashMap<Uuid,Player>, //TODO: arc-mutex me
    pub msg: Arc<Mutex<Vec<(Uuid,String)>>>, //cached messages of inbound chat
}

impl Client {
    #[allow(dead_code)]
    pub fn new (key: [u8;KEY_LEN], uuid: Uuid) -> Client {
        Client { base: ClientBase { key:From::from(&key[..]),
                                    id:uuid, },
                 stream: None,
                 cache: HashMap::new(),
                 msg: Arc::new(Mutex::new(vec!())),
        }
    }
    
    #[allow(unused_must_use)]
    pub fn default () -> Client {        
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

    pub fn load_file (path: &str) -> Option<Client> {
        let f = File::open(path);
        if let Ok(mut f) = f {
            return Client::load(&mut f)
        }

        None
    }

    pub fn load<S:Read> (s: &mut S) -> Option<Client> {
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
            self.stream = Some(Arc::new(Mutex::new(s)));
        }
        else { println!("cannot connect to server {:?}",server) }
    }

    #[allow(unused_must_use)]
    pub fn login (&mut self) -> bool {
        let mut c = self.clone();
        if let Some(ref s) = self.stream {
            if let Ok(ref mut s) = s.lock() {
                let mut m = [0u8;ID_LEN];
                if let Ok(_) = s.read_exact(&mut m) {
                    s.write_all(&[0]);
                    let hm = hmacsha1::hmac_sha1(&self.base.key, &m);
                    
                    s.write_all(&hm);
                    s.write_all(self.base.id.as_bytes());

                    
                    thread::spawn(move || {
                        c.handler()
                    });

                    return true
                }
            }
        }

        false
    }

    #[allow(unused_must_use)]
    pub fn register (&mut self) {
        if let Some(ref ms) = self.stream {
            if let Ok(ref mut s) = ms.lock() {
                s.write_all(&[1]);
                s.write_all(&self.base.key);
                s.write_all(self.base.id.as_bytes());
            }
        }
    }

    #[allow(unused_must_use)]
    pub fn chat (&mut self, text: &str) {
        if let Some(ref ms) = self.stream {
            if let Ok(ref mut s) = ms.lock() {
                write_text(s, text);
            }
        }
    }

    #[allow(unused_must_use)]
    pub fn nick (&mut self, nick: &str) {
        if let Some(ref ms) = self.stream {
            if let Ok(ref mut s) = ms.lock() {
                let (mut data, bytes) = text_as_bytes(nick);
                data[0] = 3; // specify nick route

                s.write_all(&data);
                s.write_all(bytes);
            }
        }
    }

    #[allow(unused_must_use)]
    pub fn ping (&mut self) -> bool {
        if let Some(ref ms) = self.stream {
            if let Ok(ref mut s) = ms.lock() {
                return s.write_all(&[opcode::PING]).is_ok()
            }
        }

        false
    }

    
    fn handler (&mut self) {
        let mut cmd = [0u8;1];
        let mut s; // NOTE: this should be read from only!

        if let Some(ref ms) = self.stream {
            if let Ok(s_) = ms.lock() {
                s = s_.try_clone().unwrap();
            }
            else { panic!("client stream mutex poisoned") }
        }
        else { panic!("no client stream available") }
        
        loop {
            if let Ok(_) = s.read_exact(&mut cmd) {
                match cmd[0] {
                    opcode::CHAT => {
                        if let Some(text) = read_text(&mut s) {
                            let mut id = [0u8;ID_LEN];
                            if let Ok(_) = s.read_exact(&mut id) {
                                if let Ok(uuid) = Uuid::from_bytes(&id) {
                                    if let Ok(ref mut msg) = self.msg.lock() {
                                        msg.push((uuid,text));
                                    }
                                }
                            }
                        }
                    },
                    opcode::PLAYER => {
                        if let (Some(uuid),Some(player)) = Player::from_stream(&mut s, true) {
                            println!("player:{:?} {:?}", uuid, player);
                            self.cache.insert(uuid, player);
                        }
                    },
                    opcode::PING => {
                        
                    },
                    _ => {
                        println!("unknown command {:?}",cmd)
                    },
                }
            }
        }
    }
}

impl Drop for Client {
    fn drop (&mut self) {
        if let Some(ref mut ms) = self.stream {
            if let Ok(mut s) = ms.lock() {
                let _ = s.flush();
                let _ = s.shutdown(Shutdown::Both);
            }
        }
    }
}
