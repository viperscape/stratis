extern crate getopts;
extern crate notify;


use self::notify::{Watcher, RecursiveMode, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::env;
use std::fs;
use std::thread;
use std::process::{Command,Child};

/// watches the debug build, restarts server on build
pub fn watcher (matches: &getopts::Matches) {
    let stratis_path;
    let stratis_dest;
    
    if matches.opt_present("d") {
        stratis_path = env::var("STRATIS_DEBUG").expect("no \'STRATIS_DEBUG\' path found in environment vars");
        stratis_dest = stratis_path.clone()+"\\PAN_stratis.exe";
    }
    else {
        stratis_path = env::var("STRATIS_RELEASE").expect("no \'STRATIS_RELEASE\' path found in environment vars");
        stratis_dest = stratis_path.clone()+"\\PAN_stratis.exe";
    }

    
    if matches.opt_present("w") {
        let (tx, rx) = channel();
        let mut w = self::notify::watcher(tx,Duration::from_secs(3)).expect("unable to create filesys watcher");

        w.watch(stratis_path.clone()+"\\stratis.exe",RecursiveMode::NonRecursive).expect("unable to watch directory");

        let mut spawn_handle: Option<Child> = None;
        
        for n in rx.iter() {
            let mut new_spawn_handle = None;
            println!("event: {:?}",n);
            
            match n {
                DebouncedEvent::Write(path) => {
                    if let Some(ref mut h) = spawn_handle {
                        if let Ok(_) = h.kill() {
                            println!("process-killed\n");
                            thread::sleep(Duration::from_secs(1));
                            
                            if let Ok(r) = fs::copy(path,&stratis_dest) {
                                println!("watcher-copy:{:?}",r);
                                new_spawn_handle = spawn(&stratis_dest);
                            }
                        }
                    }
                    else { //no child process alive?
                        if let Ok(r) = fs::copy(path,&stratis_dest) {
                            println!("watcher-copy:{:?}",r);
                            new_spawn_handle = spawn(&stratis_dest);
                        }
                    }

                    spawn_handle = new_spawn_handle;
                    if let Some(ref h) = spawn_handle {
                        println!("process-spawned:{:?}\n",h.id());
                    }
                },
                _ => {  },
            }
        }

    }
}

fn spawn(path: &str) -> Option<Child> {
    let r = Command::new(path).spawn();
    match r {
        Ok(h) => {
            Some(h)
        },
        Err(e) => {
            println!("spawn-err:{:?}",e);
            None
        }
    }
}
