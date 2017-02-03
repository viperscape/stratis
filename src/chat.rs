use std::net::{TcpStream};

use std::io::prelude::*;

extern crate byteorder;

use self::byteorder::{BigEndian,ByteOrder};

pub const MAX_TEXT_LEN: usize = 2048;

#[allow(unused_must_use)]
pub fn read_text (mut s: &mut TcpStream,) -> Option<String> {
    let mut size = [0;2];
    
    if let Ok(_) = s.read_exact(&mut size) {
        let size = BigEndian::read_u16(&size);

        if size > MAX_TEXT_LEN as u16 { panic!("unbounded chat size reached") }
        
        let mut v = vec![0;size as usize];
        
        if let Ok(_) = s.read_exact(&mut v) {
            if let Ok(text) = String::from_utf8(v) {
                return Some(text)
            }
        }
    }

    return None
}

#[allow(unused_must_use)]
/// defaults to CHAT opcode
pub fn write_text (mut s: &mut TcpStream, text: &str) {
    let (data, bytes) = text_as_bytes(text);
    s.write_all(&data);
    s.write_all(&bytes);
}

pub fn text_as_bytes (text: &str) -> (Vec<u8>, &[u8]) {
    let bytes = &text.as_bytes();
    
    let len;
    if bytes.len() > MAX_TEXT_LEN { len = MAX_TEXT_LEN; }
    else { len = bytes.len(); }
    
    let mut size = [0;2];
    BigEndian::write_u16(&mut size, len as u16);
    
    (vec![2, size[0],size[1]], &bytes[0..len])
}
