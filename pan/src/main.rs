extern crate getopts;
use getopts::Options;
use std::env;

mod postgres;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut opts = Options::new();
    
    opts.optflag("b", "build", "Build SQL Database");
    opts.optflag("i",  "init", "Init new SQL Database");
    opts.optopt("u", "user", "SQL user", "USER");
    opts.optopt("p", "pass", "SQL password", "PASSWORD");
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    postgres::build(matches);
}
