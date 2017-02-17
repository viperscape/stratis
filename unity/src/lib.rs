extern crate stratis_shared as shared;

use std::mem::transmute;
use std::io::Write;
use std::net::Shutdown;

use shared::client::Client;
use shared::Uuid;

#[no_mangle]
pub extern fn new_client() -> *mut Client {
    unsafe { transmute(Box::new(Client::new())) }
}

#[no_mangle]
pub extern fn default_client(key: [u8;20], uuid: [u8;16]) -> *mut Client {
    if let Ok(id) = Uuid::from_bytes(&uuid) {
        unsafe { transmute(Box::new(Client::default(key, id))) }
    }
    else { new_client() }
}

#[no_mangle]
pub extern fn drop_client(cptr: *mut Client) {
    let bc: Box<Client> = unsafe { transmute(cptr) };
    let c: Client = *bc;
    if let Some(ms) = c.stream {
        if let Ok(mut s) = ms.lock() {
            let _ = s.flush();
            let _ = s.shutdown(Shutdown::Both);
        }
    }
}

//TODO: figure out refs, or just use a cstruct perhaps?
#[no_mangle]
pub extern fn get_client_id(cptr: *mut Client) -> [u8;16] {
    let client = unsafe { & *cptr };
    let mut id = [0u8;16];
    for (i,n) in client.base.id.as_bytes().iter().enumerate() {
        id[i] = *n;
    }

    id
}

#[no_mangle]
pub extern fn get_client_key(cptr: *mut Client) -> [u8;20] {
    let client = unsafe { & *cptr };
    let mut key = [0u8;20];
    for (i,n) in client.base.key.iter().enumerate() {
        key[i] = *n;
    }

    key
}

//TODO: server-ip as arg
#[no_mangle]
pub extern fn client_connect(cptr: *mut Client) -> bool {
    let mut client = unsafe { &mut *cptr };
    client.connect("127.0.0.1:9996");
    client.stream.is_some()
}
#[no_mangle]
pub extern fn client_login(cptr: *mut Client) -> bool {
    let mut client = unsafe { &mut *cptr };
    client.login()
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
    Client::save(&client, "game/client.key")
}
#[no_mangle]
pub extern fn client_load(cptr: *mut Client) -> bool {
    let mut client = unsafe { &mut *cptr };
    if let Some(c) = Client::load_file("game/client.key") {
        client.base = c.base;
        true
    }
    else { false }
}
