extern crate rand;

use std::io::prelude::*;

use chat::{read_text,text_as_bytes};


#[derive(Debug,Clone)]
pub struct Player {
    pub nick: String,
}

impl Player {
    pub fn default () -> Player {
        let mut nick = "player_".to_string();
        nick.push_str(&rand::random::<u16>().to_string());

        Player { nick: nick }
    }

    #[allow(dead_code)]
    pub fn from_stream<S:Read> (mut s: &mut S) -> Option<Player> {
        if let Some(text) = read_text(s) {
            return Some(Player { nick: text })
        }

        None
    }

    #[allow(dead_code)]
    pub fn to_bytes (player: &Player) -> Vec<u8> {
        let (mut data, bytes) = text_as_bytes(&player.nick);
        //TODO: remove nick protocol and use Player protocol only
        //NOTE: for now use '4'
        data[0] = 4; //specify Player route in protocol
        data.extend_from_slice(bytes);

        data
    }
}
