extern crate postgres;

use self::postgres::{Connection, TlsMode};

#[derive(Debug)]
pub struct Store {
    conn: Connection,
}

impl Store {
    pub fn new () -> Option<Store> {
        if let Ok(conn) = Connection::connect("postgres://stratis:stratis@localhost",
                                              TlsMode::None) {
            return Some(Store{ conn: conn })
        }
        else { return None }
    }
}
