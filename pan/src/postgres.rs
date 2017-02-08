extern crate getopts;

extern crate postgres;
use self::postgres::{Connection, TlsMode};


pub fn build (matches: &getopts::Matches) {
    if matches.opt_present("i") {
        let user = matches.opt_str("u").unwrap_or("postgres".to_owned());
        let pass = matches.opt_str("p").expect("need password, use -p opt");
        
        let mut s = String::from("postgres://");
        s.push_str(&(user+":"+&pass+"@localhost"));
        
        let conn = Connection::connect(s,
                                       TlsMode::None).expect("cannot connect to sql");

        let build = vec![&include_bytes!("../../sql/create_login.sql")[..],
                         &include_bytes!("../../sql/create_db.sql")[..]];

        for n in build {
            let s = String::from_utf8_lossy(n);
            if let Err(e) = conn.execute(&s, &[]) {
                println!("build:{:?}\nfor:{:?}\n\n",e,s);
            }
        }
    }

    if matches.opt_present("b") {
        let conn = Connection::connect("postgres://stratis:stratis@localhost",
                                       TlsMode::None).expect("cannot connect to sql");

        let build = vec![&include_bytes!("../../sql/create_players.sql")[..],
                         &include_bytes!("../../sql/create_msg.sql")[..],
                         &include_bytes!("../../sql/create_clients.sql")[..],
                         &include_bytes!("../../sql/grant_stratis.sql")[..]];
        
        for n in build {
            let s = String::from_utf8_lossy(n);
            if let Err(e) = conn.execute(&s, &[]) {
                println!("build:{:?}\nfor:{:?}\n\n",e,s);
            }
        }
    }

}
