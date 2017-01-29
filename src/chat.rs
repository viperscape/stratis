use std::net::{TcpStream};

use std::io::prelude::*;
use std::io::BufReader;

extern crate byteorder;

use self::byteorder::{BigEndian,ByteOrder};
use std::string::FromUtf8Error;

pub const MAX_TEXT_LEN: usize = 2048;


pub fn read_text (mut s: &mut TcpStream,) -> Option<String> {
    let mut size = [0;2];
    
    if let Ok(_) = s.read_exact(&mut size) {
        let size = BigEndian::read_u16(&size);
        let mut v = vec![0;size as usize];
        
        if let Ok(_) = s.read_exact(&mut v) {
            if let Ok(text) = String::from_utf8(v) {
                return Some(text)
            }
        }
    }

    return None
}

pub fn write_text (mut s: &mut TcpStream, text: &str) {
    let mut bytes = &text.as_bytes();
    
    let len;
    if bytes.len() > MAX_TEXT_LEN { len = MAX_TEXT_LEN; }
    else { len = bytes.len(); }
    
    let mut size = [0;2];
    BigEndian::write_u16(&mut size, len as u16);
    
    
    s.write_all(&[2]);
    s.write_all(&size);
    s.write_all(&bytes[0..len]);
}

// NOTE: may be better to return a tuple with the bytes slice separate from vec
pub fn text_as_bytes (text: &str) -> Vec<u8> {
    let mut bytes = &text.as_bytes();
    
    let len;
    if bytes.len() > MAX_TEXT_LEN { len = MAX_TEXT_LEN; }
    else { len = bytes.len(); }
    
    let mut size = [0;2];
    BigEndian::write_u16(&mut size, len as u16);
    
    let mut v = vec!(2, size[0],size[1]);
    v.extend_from_slice(&bytes[0..len]);

    v
}
