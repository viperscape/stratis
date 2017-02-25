extern crate stratis_shared as shared;

use std::mem::transmute;

use std::os::raw::c_char;
use std::ffi::{CStr};
use std::str;
use std::str::Utf8Error;

use shared::client::{Client,KEY_LEN,ID_LEN};
use shared::Uuid;
use shared::chat::MAX_TEXT_LEN;

/// managed for c-interop
#[repr(C)]
pub struct MClientBase {
    id: [u8;ID_LEN],
    key: [u8;KEY_LEN],
}

#[repr(C)]
pub struct MChatFrame {
    id: [u8;ID_LEN],
    msg: [u8;MAX_TEXT_LEN],
}

fn str_from_ptr<'a> (s: *const c_char) -> Result<&'a str,Utf8Error> {
    let cstr = unsafe { CStr::from_ptr(s) };
    str::from_utf8(cstr.to_bytes())
}

#[no_mangle]
pub extern fn default_client() -> *mut Client {
    unsafe { transmute(Box::new(Client::default())) }
}

#[no_mangle]
pub extern fn new_client(key: [u8;KEY_LEN], uuid: [u8;ID_LEN]) -> *mut Client {
    if let Ok(id) = Uuid::from_bytes(&uuid) {
        unsafe { transmute(Box::new(Client::new(key, id))) }
    }
    else { default_client() }
}

#[no_mangle]
pub extern fn drop_client(cptr: *mut Client) -> u8 {
    if cptr.is_null() { return true as u8 }
    
    unsafe { Box::from_raw(cptr); }

    cptr.is_null() as u8
}


#[no_mangle]
pub extern fn get_client_base(cptr: *mut Client, cb: &mut MClientBase) {
    let client = unsafe { &mut *cptr };

    let mut key = [0u8;KEY_LEN];
    for (i,n) in client.key().iter().enumerate() {
        key[i] = *n;
    }
    
    cb.id = client.base.id.as_bytes().clone();
    cb.key = key;
}


#[no_mangle]
pub extern fn client_connect(cptr: *mut Client, s: *const c_char) -> u8 {
    let mut client = unsafe { &mut *cptr };
    if let Ok(s) = str_from_ptr(s) {
        client.connect(s);
    }
    client.stream.is_some() as u8
}
#[no_mangle]
pub extern fn client_login(cptr: *mut Client) -> u8 {
    let mut client = unsafe { &mut *cptr };
    client.login() as u8
}
#[no_mangle]
pub extern fn client_register(cptr: *mut Client) {
    let mut client = unsafe { &mut *cptr };
    client.register();
}


/// these are highly OS dependent
#[no_mangle]
pub extern fn client_save(cptr: *mut Client) -> bool {
    let client = unsafe { & *cptr };
    Client::save(&client, "client.key")
}
#[no_mangle]
pub extern fn client_load(cptr: *mut Client) -> bool {
    let mut client = unsafe { &mut *cptr };
    if let Some(c) = Client::load_file("client.key") {
        client.base = c.base.clone();
        true
    }
    else { false }
}


#[no_mangle]
pub extern fn client_chat(cptr: *mut Client, s: *const c_char) {
    let mut client = unsafe { &mut *cptr };
    if let Ok(s) = str_from_ptr(s) {
        client.chat(s);
    }
}
#[no_mangle]
pub extern fn client_nick(cptr: *mut Client, s: *const c_char) {
    let mut client = unsafe { &mut *cptr };
    if let Ok(s) = str_from_ptr(s) {
        client.nick(s);
    }
}




//NOTE: this is meant to be polled on frame-tick
#[no_mangle]
pub extern fn get_client_chat(cptr: *mut Client, chat: &mut MChatFrame) -> u16 {
    let client = unsafe { &mut *cptr };

    if let Ok(mut v) = client.msg.lock() {
        if v.len() > 0 {
            let (uuid, msg) = v.remove(0);
            
            chat.id = uuid.as_bytes().clone();

            let bytes = msg.as_bytes();
            for (i,b) in bytes.iter().enumerate() {
                chat.msg[i] = *b;
            }

            return bytes.len() as u16
        }
    }

    0
}
