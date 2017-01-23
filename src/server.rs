use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;

use client::Client;

pub struct Server;

impl Server {
    pub fn new(ip: &str) -> Server {
        let listener = TcpListener::bind(ip).unwrap();
        for s in listener.incoming() {
            match s {
                Ok(s) => {
                    thread::spawn(|| Server::handler(s));
                },
                _ => {},
            }
        }

        Server
    }

    fn handler (mut s: TcpStream) {
        let mut cmd = [0;1];
        loop {
            if let Ok(_) = s.read_exact(&mut cmd) {
                match cmd[0] {
                    0 => { //login
                        let c = Client::load(&mut s);
                        println!("client-login:{:?}",c);
                    },
                    _ => panic!("cmd:{:?}",cmd)
                }
            }
        }

        println!("client dropped");
    }
}
