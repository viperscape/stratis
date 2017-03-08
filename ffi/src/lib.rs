extern crate stratis_shared as shared;


use std::os::raw::c_char;
use std::ffi::{CStr};
use std::str;
use std::str::Utf8Error;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;

use shared::client::{KEY_LEN,ID_LEN, Client};
use shared::chat::MAX_TEXT_LEN;
use shared::player::MAX_NICK_LEN;
use shared::events::Event;

/// managed for c-interop
#[repr(C)]
pub struct MClient { // we're coupling the channel rx and the client here
    client: *mut Arc<Mutex<Client>>,
    rx: *mut Receiver<Event>
}

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

#[repr(C)]
pub struct MPlayer {
    nick: [u8;MAX_NICK_LEN],
}

// TODO: verify lifetime is actually valid here, conv to String instead?
fn str_from_ptr<'a> (s: *const c_char) -> Result<&'a str,Utf8Error> {
    let cstr = unsafe { CStr::from_ptr(s) };
    str::from_utf8(cstr.to_bytes())
}

pub mod client;
pub mod timer;
pub mod events;
