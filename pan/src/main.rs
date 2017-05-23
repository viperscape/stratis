extern crate getopts;
use getopts::Options;
use std::env;

mod postgres;
mod admin;
mod lifecycle;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut opts = Options::new();
    
    opts.optflag("b", "build", "Build SQL Database");
    opts.optflag("i",  "init", "Init new SQL Database");
    opts.optflag("f",  "force", "Force option");
    
    opts.optopt("u", "user", "SQL user", "USER");
    opts.optopt("p", "pass", "SQL password", "PASSWORD");
    

    opts.optflag("w",  "watch", "Watch and rerun stratis builds");
    opts.optflag("d", "debug", "Specify debug stratis build");

    opts.optflag("r",  "run", "Run server immediately");

    opts.optflag("c", "create", "Create admin user");
    
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    postgres::build(&matches);
    admin::create(&matches);
    lifecycle::watcher(&matches);
}
