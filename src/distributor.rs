extern crate uuid;

use std::sync::mpsc::{channel, Sender, Receiver};
use std::collections::{HashMap};
use std::io::Write;

use self::uuid::Uuid;
use store::DataStore;

/// a distribution service
pub struct Distributor<S:Write, D:DataStore> {
    rx: Receiver<Kind<S>>, //receiver ch with data to be redistrubuted
    sx: HashMap<Uuid,S>, //local cache of streams to comm to
    store: D,
}

impl<S:Write, D:DataStore> Distributor<S,D>  {
    pub fn new (store: D) -> (Sender<Kind<S>>, Distributor<S,D>) {
        let (tx,rx) = channel();
        (tx, //sending channel to comm with Distributor
         Distributor {
             rx: rx,
             sx: HashMap::new(),
             store: store,
         })
    }

    pub fn run (&mut self) {
        let mut dead: Vec<Uuid> = vec!();
        
        for n in self.rx.iter() {
            match n {
                Kind::Broadcast(data) => {
                    for (uuid,mut stream) in self.sx.iter_mut() {
                        Distributor::<S,D>::write(stream,&uuid,&data,&mut dead, &self.store);
                    }
                },
                Kind::Select(uuid,data) => {
                    if let Some(stream) = self.sx.get_mut(&uuid) {
                        Distributor::<S,D>::write(stream,&uuid,&data,&mut dead, &self.store);
                    }
                },
                Kind::Group(mut uuids,data) => {
                    for uuid in uuids.drain(..) {
                        if let Some(stream) = self.sx.get_mut(&uuid) {
                            Distributor::<S,D>::write(stream,&uuid,&data,&mut dead, &self.store);
                        }
                    }
                },
                Kind::Add(uuid,stream) => {
                    self.sx.insert(uuid,stream);

                    // retrieve cached messages while away
                    for msg in self.store.msg_get(&uuid) {
                        let stream = self.sx.get_mut(&uuid).unwrap();
                        Distributor::<S,D>::write(stream,&uuid, &msg,
                                                  &mut dead, &self.store);
                    }
                }
                Kind::Remove(uuid) => { self.sx.remove(&uuid); }
            }

            // remove dead streams
            for n in dead.drain(..) {
                self.sx.remove(&n);
            }
        }
    }

    // FIXME: cannot use &mut self here!
    fn write (stream: &mut S,
              uuid: &Uuid,
              data: &Vec<u8>,
              dead: &mut Vec<Uuid>,
              store: &D) {
        if stream.write_all(data).is_err() {
            dead.push(uuid.clone());
            store.msg_put(uuid, data);
        }
    }
}

#[allow(dead_code)]
pub enum Kind<S> {
    Add(Uuid,S),
    Remove(Uuid),
    
    Broadcast(Vec<u8>), //broadcast style data
    Select(Uuid,Vec<u8>), //send to single stream
    Group(Vec<Uuid>,Vec<u8>), //send to few streams
}
