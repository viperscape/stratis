extern crate getopts;
extern crate stratis_shared as shared;

use self::shared::client::Client;
use ::postgres::sql_exec;

pub fn create (matches: &getopts::Matches) {
    if matches.opt_present("c") {
        let ip_addr = "127.0.0.1:9996";
        
        let (mut c, _) = Client::default();
        c.connect(ip_addr);
        if c.stream.is_none() { panic!("Client unable to connect to server") }
        c.register();

        
        let r = sql_exec(matches, "Update Clients set is_admin = true where UUID = '$1';", &[&c.id().to_string()]);
        if r.is_err() {
            panic!("Unable to set user as admin {:?}", r)
        }

        Client::save(&c,"admin.key");
    }
}
