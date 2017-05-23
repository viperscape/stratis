extern crate getopts;
extern crate stratis_shared as shared;

use self::shared::client::Client;

pub fn create (matches: &getopts::Matches) {
    if matches.opt_present("i") {
        let ip_addr = "127.0.0.1:9996";
        
        let (mut c, _) = Client::default();
        Client::save(&c,"admin.key");
        c.connect(ip_addr);
        c.register();
    }

}
