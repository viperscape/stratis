extern crate getopts;
extern crate notify;


use self::notify::{Watcher, RecursiveMode};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::env;

/// watches the debug build, restarts server on build
pub fn watcher (matches: &getopts::Matches) {
    if matches.opt_present("w") && matches.opt_present("d") {
        
        let stratis_path = env::var("STRATIS_DEBUG").expect("no \'STRATIS_DEBUG\' path found in environment vars");
        
        let (tx, rx) = channel();
        let mut w = self::notify::watcher(tx,Duration::from_secs(5)).expect("unable to create filesys watcher");

        w.watch(stratis_path,RecursiveMode::NonRecursive).expect("unable to watch directory");

        loop {
            match rx.recv() {
                Ok(event) => println!("{:?}", event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }

    }
}
