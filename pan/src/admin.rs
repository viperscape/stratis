use shared::client::Client;
use server::{Store,DataStore};
use getopts;

use db::sql_exec;

pub fn create (matches: &getopts::Matches) {
    if matches.opt_present("c") {
        let (c, _) = Client::default();
        let store = Store::default();
        store.client_put(&c.base);
        
        let r = sql_exec(matches, "update clients set is_admin = true where UUID = $1;", &[&c.id()]);
        if r.is_err() {
            panic!("Unable to set user as admin {:?}", r)
        }

        Client::save(&c,"admin.key");
    }
}
