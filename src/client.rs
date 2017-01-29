extern crate hmacsha1;
extern crate uuid;
extern crate byteorder;

use self::uuid::Uuid;

use std::io::prelude::*;
//use std::io::BufReader;
use std::fs::File;
use std::net::TcpStream;

use std::thread;
use std::sync::{Arc, Mutex};


use chat::{read_text,write_text};


#[derive(Debug,Clone)]
pub struct Client {
    pub key: [u8;20],
    pub id: Uuid,
    pub stream: Option<Arc<Mutex<TcpStream>>>,
}

impl Client {
    #[allow(unused_must_use)]
    pub fn new (path: &str) -> Client { println!("new file");
        let id = uuid::Uuid::new_v4();
        let m = hmacsha1::hmac_sha1(uuid::Uuid::new_v4().as_bytes(),
                                    id.as_bytes());

        
        let c = Client { id: id, key: m, stream: None };

        let f = File::create(path);
        if !f.is_ok() { panic!("cannot create client file") }
        if let Ok(mut f) = f {
            f.write_all(&c.key);
            f.write_all(c.id.as_bytes());
        }

        c
    }

    pub fn load_file (path: &str) -> Option<Client> {
        let f = File::open(path);
        if let Ok(mut f) = f {
            return Client::load(&mut f)
        }

        None
    }

    pub fn load<S:Read> (s: &mut S) -> Option<Client> {
        let mut key = [0u8;20];
        let mut id = [0u8;16];
        if let Ok(_) = s.read_exact(&mut key) {
            if let Ok(_) = s.read_exact(&mut id) {
                if let Ok(id) = Uuid::from_bytes(&id) {
                    return Some(Client { id:id, key: key, stream:None })
                }
                else { println!("cannot uuid file") }
            }
        }

        None
    }
    
    pub fn connect (&mut self, server: &str)  {
        if let Ok(s) = TcpStream::connect(server) {
            self.stream = Some(Arc::new(Mutex::new(s)));
        }
        else { panic!("cannot connect to server {:?}",server) }
    }

    #[allow(unused_must_use)]
    pub fn login (&mut self) {
        let mut c = self.clone();
        if let Some(ref s) = self.stream {
            if let Ok(ref mut s) = s.lock() {
                let mut m = [0u8;16];
                if let Ok(_) = s.read_exact(&mut m) {
                    s.write_all(&[0]);
                    let hm = hmacsha1::hmac_sha1(&self.key, &m);
                    
                    s.write_all(&hm);
                    s.write_all(self.id.as_bytes());

                    
                    thread::spawn(move || {
                        c.handler()
                    });
                }
            }
        }
    }

    #[allow(unused_must_use)]
    pub fn register (&mut self) {
        if let Some(ref ms) = self.stream {
            if let Ok(ref mut s) = ms.lock() {
                s.write_all(&[1]);
                s.write_all(&self.key);
                s.write_all(self.id.as_bytes());
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
                    2 => {
                        if let Some(text) = read_text(&mut s) {
                            println!("chat-server:{:?}",text.trim());
                        }
                    },
                    _ => {
                        println!("unknown command {:?}",cmd)
                    },
                }
            }
        }
    }
}

