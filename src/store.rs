extern crate postgres;

use self::postgres::{Connection, TlsMode};
use client::Client;

#[derive(Debug)]
pub struct Store {
    pub conn: Connection,
}

impl Store {
    pub fn new () -> Option<Store> {
        if let Ok(conn) = Connection::connect("postgres://stratis:stratis@localhost",
                                              TlsMode::None) {
            return Some(Store{ conn: conn })
        }
        else { return None }
    }

    pub fn get_clients (&self) -> Vec<Client> {
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

    pub fn add_client (&self, c: &Client) -> bool {
        self.conn.execute("INSERT INTO clients (uuid, key) VALUES ($1, $2)",
                          &[&c.id, &c.key]).is_ok()
    }
}
