extern crate postgres;
extern crate uuid;

use self::postgres::{Connection, TlsMode};
use client::Client;
use self::uuid::Uuid;

pub trait DataStore: Sized {
    fn default () -> Option<Self>;
    fn get_clients (&self) -> Vec<Client>;
    fn add_client (&self, c: &Client) -> bool;
    fn msg_store (&self, uuid: &Uuid, data: &Vec<u8>) -> bool;
    fn get_msg (&self, uuid: &Uuid) -> Vec<Vec<u8>>;
}

#[derive(Debug)]
pub struct Store {
    pub conn: Connection,
}

impl DataStore for Store {
    fn default () -> Option<Store> {
        if let Ok(conn) = Connection::connect("postgres://stratis:stratis@localhost",
                                              TlsMode::None) {
            return Some(Store{ conn: conn })
        }
        else { return None }
    }

    fn get_clients (&self) -> Vec<Client> {
        let mut clients = vec!();
        if let Ok(r) = self.conn.query("select * from clients",&[]) {
            for n in r.iter() {
                clients.push(
                    Client { key:n.get(1),
                             id:n.get(0),
                             stream: None, });
            }
        }

        clients
    }

    fn add_client (&self, c: &Client) -> bool {
        self.conn.execute("INSERT INTO clients (uuid, key) VALUES ($1, $2)",
                          &[&c.id, &c.key]).is_ok()
    }

    fn msg_store (&self, uuid: &Uuid, data: &Vec<u8>) -> bool {
        self.conn.execute("INSERT INTO msg (uuid, msg) VALUES ($1, $2)",
                          &[uuid, data]).is_ok()
    }
    fn get_msg (&self, uuid: &Uuid) -> Vec<Vec<u8>> {
        let mut msgs = vec!();
        if let Ok(r) = self.conn.query("select msg from msg where uuid = $1",&[uuid]) {
            for msg in r.iter() {
                msgs.push(msg.get(0));
            }
        }

        msgs
    }
}
