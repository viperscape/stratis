extern crate uuid;


use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::collections::{HashMap};
use std::io::Write;

use self::uuid::Uuid;

/// a distribution service
pub struct Distributor<S:Write> {
    rx: Receiver<Kind<S>>, //receiver ch with data to be redistrubuted
    sx: HashMap<Uuid,S>, //local cache of streams to comm to
}

impl<S:Write> Distributor<S>  {
    pub fn new () -> (Sender<Kind<S>>, Distributor<S>) {
        let (tx,rx) = channel();
        (tx, //sending channel to comm with Distributor
         Distributor {
             rx: rx,
             sx: HashMap::new(),
         })
    }

    pub fn run (&mut self) {
        for n in self.rx.iter() {
            match n {
                Kind::Broadcast(data) => {
                    for (uuid,mut stream) in self.sx.iter_mut() {
                        stream.write_all(&data);
                    }
                },
                Kind::Select(uuid,data) => {
                    if let Some(stream) = self.sx.get_mut(&uuid) {
                        stream.write_all(&data);
                    }
                },
                Kind::Group(mut uuids,data) => {
                    for uuid in uuids.drain(..) {
                        if let Some(stream) = self.sx.get_mut(&uuid) {
                            stream.write_all(&data);
                        }
                    }
                },
                Kind::Add(uuid,stream) => { self.sx.insert(uuid,stream); }
                Kind::Remove(uuid) => { self.sx.remove(&uuid); }
            }
        }
    }
}

pub enum Kind<S> {
    Add(Uuid,S),
    Remove(Uuid),
    
    Broadcast(Vec<u8>), //broadcast style data
    Select(Uuid,Vec<u8>), //send to single stream
    Group(Vec<Uuid>,Vec<u8>), //send to few streams
}
