extern crate stratis_shared as shared;
extern crate uuid;

use ::uuid::Uuid;

use std::io::prelude::*;
use std::io::Cursor;

use shared::player::Player;
use shared::opcode;

use shared::chat;

/// recreate to appease type checker
fn cursor_transform (s: &mut Cursor<Vec<u8>>) -> Cursor<&mut [u8]> {
 Cursor::new(&mut s.get_mut()[..])
}

fn assert_opcode(s: &mut Cursor<&mut [u8]>, op: u8) {
    s.set_position(0);
    let mut b = [0u8];
    let r = s.read_exact(&mut b);
    assert!(r.is_ok());
    assert_eq!(b[0], op);
}

#[test]
fn player_uuid () {
    let p = Player::default();
    let id = Uuid::new_v4();
    
    let mut bytes = Player::to_bytes(Some(&id),&p);
    let mut s = Cursor::new(&mut bytes[..]);

    assert!(s.get_ref().len() > 0);

    assert_opcode(&mut s,opcode::PLAYER);
    
    let r = Player::from_stream(&mut s, true);
    assert!(r.0.is_some());
    assert!(r.1.is_some());
    
    assert_eq!(id,r.0.unwrap());
    assert_eq!(p,r.1.unwrap());
}

#[test]
fn player () {
    let p = Player::default();
    
    let mut bytes = Player::to_bytes(None,&p);
    let mut s = Cursor::new(&mut bytes[..]);

    assert!(s.get_ref().len() > 0);

    assert_opcode(&mut s,opcode::PLAYER);
    
    let r = Player::from_stream(&mut s, false);
    assert!(r.0.is_none());
    assert!(r.1.is_some());

    assert_eq!(p,r.1.unwrap());
}


#[test]
fn chat() {
    let text = "test";
    let mut s = Cursor::new(vec![]);
    chat::write_text(&mut s, text);
    
    let mut s = cursor_transform(&mut s);
    assert!(s.get_ref().len() > 0);
    assert_opcode(&mut s,opcode::CHAT);
    
    let r = chat::read_text(&mut s);
    assert!(r.is_some());

    assert_eq!(text, r.unwrap());
}
