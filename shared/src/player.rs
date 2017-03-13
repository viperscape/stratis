extern crate rand;
extern crate uuid;

use std::io::prelude::*;

use chat::{read_text,text_as_bytes};
use self::uuid::Uuid;
use opcode;

pub const MAX_NICK_LEN: usize = 30;


#[derive(Debug,Clone,PartialEq)]
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
    pub fn from_stream<S:Read> (mut s: &mut S, get_uuid: bool) -> (Option<Uuid>, Option<Player>) {
        if let Some(text) = read_text(s) {
            let mut nick = text.trim().to_owned();
            nick.truncate(MAX_NICK_LEN);
            
            let p = Some(Player { nick: nick });
            let i;
            if !get_uuid { i = None }
            else {
                let mut id = [0u8;::client::ID_LEN];
                if let Ok(_) = s.read_exact(&mut id) {
                    if let Ok(uuid) = Uuid::from_bytes(&id) {
                        i = Some(uuid);
                    }
                    else { i = None }
                }
                else { i = None } 
            }
            
            return (i, p)
        }

        (None,None)
    }

    #[allow(dead_code)]
    pub fn to_bytes (uuid: Option<&Uuid>, player: &Player) -> Vec<u8> {
        let (mut data, bytes) = text_as_bytes(&player.nick);
        
        data[0] = opcode::PLAYER; //specify Player route in protocol
        data.extend_from_slice(bytes);

        if let Some(uuid) = uuid { //NOTE: server sends uuid to client
            data.extend_from_slice(uuid.as_bytes()); //refer to uuid
        }
        
        data
    }
}
