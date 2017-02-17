extern crate getopts;
extern crate notify;


use self::notify::{Watcher, RecursiveMode, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::env;
use std::fs;
use std::path;
use std::thread;
use std::process::{Command,Child};

/// watches the debug build, restarts server on build
pub fn watcher (matches: &getopts::Matches) {
    if !matches.opt_present("w") { return }
    
    let stratis_project = env::var("STRATIS").expect("no \'STRATIS\' path found in environment vars");
    let stratis_path;
    let stratis_dest;
    
    let unity_path;
    let unity_dest = env::var("STRATIS_UNITY");
    
    if matches.opt_present("d") {
        stratis_path = stratis_project.clone() + "\\server\\target\\debug\\";
        stratis_dest = stratis_path.clone()+"PAN_stratis.exe";

        unity_path = stratis_project.clone()+"\\unity\\target\\debug";
    }
    else {
        stratis_path = stratis_project.clone() + "\\server\\target\\release\\";
        stratis_dest = stratis_path.clone()+"\\PAN_stratis.exe";

        unity_path = stratis_project.clone()+"\\unity\\target\\release";
    }

    

    
    let (tx, rx) = channel();
    let mut w = self::notify::watcher(tx,Duration::from_secs(1)).expect("unable to create filesys watcher");

    w.watch(stratis_path.clone()+"\\stratis.exe",RecursiveMode::NonRecursive);
    
    if let Ok(ref dest) = unity_dest {
        println!("watching stratis_unity builds for:{:?}",dest);
        w.watch(unity_path.clone()+"\\stratis_unity.dll",RecursiveMode::NonRecursive);
    }

    
    let mut spawn_handle: Option<Child> = None;
    if matches.opt_present("r") {
        
        if Command::new("cargo")
            .current_dir(stratis_project.clone() + "server")
            .arg("build")
            .status().expect("failed to build stratis server").success() {
                spawn_handle = spawn(&stratis_dest);
            }
    }
    
    for n in rx.iter() {
        let mut new_spawn_handle = None;
        println!("event: {:?}",n);
        
        match n {
            DebouncedEvent::Write(path) => {
                if path == path::PathBuf::from(stratis_path.clone()+"stratis.exe") {
                    if let Some(ref mut h) = spawn_handle {
                        if let Ok(_) = h.kill() {
                            println!("process-killed\n");
                            thread::sleep(Duration::from_secs(1));
                            
                            if let Ok(r) = fs::copy(&path,&stratis_dest) {
                                println!("watcher-copy:{:?}",r);
                                new_spawn_handle = spawn(&stratis_dest);
                            }
                        }
                    }
                    else { //no child process alive?
                        if let Ok(r) = fs::copy(&path,&stratis_dest) {
                            println!("watcher-copy:{:?}",r);
                            new_spawn_handle = spawn(&stratis_dest);
                        }
                    }

                    spawn_handle = new_spawn_handle;
                    if let Some(ref h) = spawn_handle {
                        println!("process-spawned:{:?}\n",h.id());
                    }
                }
                else if path == path::PathBuf::from(unity_path.clone()+"\\stratis_unity.dll") {
                    if let Ok(ref unity_dest) = unity_dest {
                        println!("copying unity dll");
                        let r = fs::copy(&path,unity_dest.clone()+"\\Assets\\Plugins\\stratis_unity.dll");
                        println!("watcher-copy:{:?}",r);
                    }
                }
            },
            _ => {  },
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
