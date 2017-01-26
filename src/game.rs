#![allow(dead_code)]
#![allow(unused_must_use)]

extern crate std;
extern crate hmacsha1;

use std::fs::File;

pub struct Game {
    file: std::fs::File,
}

impl Game {
    pub fn new () -> Game {
        std::fs::create_dir("game");
        let f = File::create("game/game.log");
        if !f.is_ok() { panic!("cannot create game log") }

        Game { file: f.unwrap() }
    }

    
}
