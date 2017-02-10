extern crate postgres;
extern crate uuid;

use self::postgres::{Connection, TlsMode};
use client::{Client};
use self::uuid::Uuid;

pub trait DataStore: Sized {
    fn default () -> Self;
    fn clients_get (&self) -> Vec<Client>;
    fn client_put (&self, c: &Client) -> bool;
    fn msg_put (&self, uuid: &Uuid, data: &Vec<u8>) -> bool;
    fn msg_get (&self, uuid: &Uuid) -> Vec<Vec<u8>>;
    fn player_get (&self, uuid: &Uuid) -> Option<String>;
    fn player_put (&self, uuid: &Uuid, nick: String) -> bool;
}

#[derive(Debug)]
pub struct Store {
    pub conn: Connection,
}

impl DataStore for Store {
    fn default () -> Store {
        let conn = Connection::connect("postgres://stratis:stratis@localhost",
                                       TlsMode::None).expect("unable to connect to postgres");
        Store { conn: conn }
    }

    fn clients_get (&self) -> Vec<Client> {
        let mut clients = vec!();
        if let Ok(r) = self.conn.query("select * from clients",&[]) {
            for n in r.iter() {
                clients.push(Client::default(n.get(1),n.get(0)));
            }
        }

        clients
    }

    fn client_put (&self, c: &Client) -> bool {
        self.conn.execute("INSERT INTO clients (uuid, key) VALUES ($1, $2)",
                          &[c.id(),&c.key_as_ref()]).is_ok()
    }

    fn msg_put (&self, uuid: &Uuid, data: &Vec<u8>) -> bool {
        self.conn.execute("INSERT INTO msg (uuid, msg) VALUES ($1, $2)",
                          &[uuid, data]).is_ok()
    }
    fn msg_get (&self, uuid: &Uuid) -> Vec<Vec<u8>> {
        let mut msgs = vec!();
        if let Ok(r) = self.conn.query("select msg from msg where uuid = $1",&[uuid]) {
            for msg in r.iter() {
                msgs.push(msg.get(0));
            }
        }

        msgs
    }

    fn player_get (&self, uuid: &Uuid) -> Option<String> {
        if let Ok(r) = self.conn.query("select nick from players where uuid = $1 limit 1",&[uuid]) {
            if let Some(r) = r.iter().next() {
                return r.get(0)
            }
        }

        None
    }
    fn player_put (&self, uuid: &Uuid, nick: String) -> bool {
        let r = self.conn.execute("INSERT INTO players (uuid, nick) VALUES ($1, $2)",
                                  &[uuid, &nick]);
        println!("{:?}",r);

        r.is_ok()
    }
}
