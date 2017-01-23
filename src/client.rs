extern crate hmacsha1;
extern crate uuid;

use self::uuid::Uuid;

use std::io::prelude::*;
use std::fs::File;
use std::net::TcpStream;


#[derive(Debug)]
pub struct Client {
    id: Uuid,
    key: [u8;20],
}

impl Client {
    pub fn new (path: &str) -> Client {
        let id = uuid::Uuid::new_v4();
        let m = hmacsha1::hmac_sha1(id.as_bytes(),
                                    uuid::Uuid::new_v4().as_bytes());

        
        let c = Client { id: id, key: m };

        let mut f = File::create(path);
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
                    return Some(Client { id:id, key: key })
                }
            }
        }

        None
    }

    pub fn connect (&mut self, server: &str) {
        if let Ok(mut s) = TcpStream::connect(server) {
            s.write_all(&[0]);
            s.write_all(&self.key);
            s.write_all(self.id.as_bytes());
            //s.flush();
        }
        else { panic!("cannot connect to server {:?}",server) }
    }
}

