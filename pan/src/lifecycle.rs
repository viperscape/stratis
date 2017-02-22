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
    let server_path;
    let server_dest;
    
    let ffi_path;
    let ffi_dest = env::var("STRATIS_LINK");
    
    if matches.opt_present("d") {
        server_path = stratis_project.clone() + "\\server\\target\\debug\\";
        server_dest = server_path.clone()+"PAN_stratis.exe";

        ffi_path = stratis_project.clone()+"\\ffi\\target\\debug\\";
    }
    else {
        server_path = stratis_project.clone() + "\\server\\target\\release\\";
        server_dest = server_path.clone()+"\\PAN_stratis.exe";

        ffi_path = stratis_project.clone()+"\\ffi\\target\\release\\";
    }

    

    
    let (tx, rx) = channel();
    let mut w = self::notify::watcher(tx,Duration::from_secs(1)).expect("unable to create filesys watcher");

    let _ = w.watch(server_path.clone()+"stratis.exe",RecursiveMode::NonRecursive);
    
    if let Ok(ref dest) = ffi_dest {
        println!("watching stratis_ffi builds for:{:?}",dest);
        let _ = w.watch(ffi_path.clone()+"stratis_ffi.dll",RecursiveMode::NonRecursive);
    }

    
    let mut spawn_handle: Option<Child> = None;
    if matches.opt_present("r") {
        
        if Command::new("cargo")
            .current_dir(stratis_project.clone() + "\\server\\")
            .arg("build")
            .status().expect("failed to build stratis server").success() {
                spawn_handle = spawn(&server_dest);
            }
    }
    
    for n in rx.iter() {
        let mut new_spawn_handle = None;
        println!("event: {:?}",n);
        
        match n {
            DebouncedEvent::Write(path) => {
                if path == path::PathBuf::from(server_path.clone()+"stratis.exe") {
                    if let Some(ref mut h) = spawn_handle {
                        if let Ok(_) = h.kill() {
                            println!("process-killed\n");
                            thread::sleep(Duration::from_secs(1));
                            
                            if let Ok(r) = fs::copy(&path,&server_dest) {
                                println!("watcher-copy:{:?}",r);
                                new_spawn_handle = spawn(&server_dest);
                            }
                        }
                    }
                    else { //no child process alive?
                        if let Ok(r) = fs::copy(&path,&server_dest) {
                            println!("watcher-copy:{:?}",r);
                            new_spawn_handle = spawn(&server_dest);
                        }
                    }

                    spawn_handle = new_spawn_handle;
                    if let Some(ref h) = spawn_handle {
                        println!("process-spawned:{:?}\n",h.id());
                    }
                }
                else if path == path::PathBuf::from(ffi_path.clone()+"stratis_ffi.dll") {
                    if let Ok(_) = ffi_dest { //TODO: copy dll to STRATIS_LINK path?
                        println!("copying ffi dll");
                        let r = fs::copy(&path,stratis_project.clone()+"\\unity_ffi\\stratis_ffi.dll"); // we do this because VS is stupid
                        println!("watcher-copy:{:?}",r);
                        
                        let r = fs::copy(&path,stratis_project.clone()+"\\unity_ffi\\Assets\\Plugins\\stratis_ffi.dll");
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
