extern crate getopts;
use getopts::Options;
use std::env;

extern crate postgres;
use self::postgres::{Connection, TlsMode};


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("b", "Build SQL Database", "build");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    
    let build = vec![&include_bytes!("../../sql/grant_stratis.sql")[..],
                     &include_bytes!("../../sql/create_players.sql")[..],
                     &include_bytes!("../../sql/create_msg.sql")[..],
                     &include_bytes!("../../sql/create_clients.sql")[..]];

    if matches.opt_present("b") {
        let conn = Connection::connect("postgres://stratis:stratis@localhost",
                                       TlsMode::None).expect("cannot connect to sql");
        for n in build {
            let s = String::from_utf8_lossy(n);
            if let Err(e) = conn.execute(&s, &[]) {
                println!("build:{:?}\nfor:{:?}\n\n",e,s);
            }
        }
    }
}
