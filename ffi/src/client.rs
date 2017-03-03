use std::mem::transmute;
use std::sync::{Arc, Mutex};

use shared::client::Client;

use ::{MClientBase,MChatFrame,KEY_LEN,
       c_char,str_from_ptr};


#[no_mangle]
pub extern fn default_client() -> *mut Arc<Mutex<Client>> {
    unsafe { transmute(Box::new(Arc::new(Mutex::new(Client::default())))) }
}


#[no_mangle]
pub extern fn drop_client(cptr: *mut Arc<Mutex<Client>>) -> u8 {
    if cptr.is_null() { return true as u8 }
    
    unsafe { Box::from_raw(cptr); }

    cptr.is_null() as u8
}


#[no_mangle]
pub extern fn get_client_base(cptr: *mut Arc<Mutex<Client>>, cb: &mut MClientBase) {
    let client = unsafe { & *cptr };
    let client = client.lock().unwrap();

    let mut key = [0u8;KEY_LEN];
    for (i,n) in client.key().iter().enumerate() {
        key[i] = *n;
    }
    
    cb.id = client.base.id.as_bytes().clone();
    cb.key = key;
}


#[no_mangle]
pub extern fn client_connect(cptr: *mut Arc<Mutex<Client>>, s: *const c_char) -> u8 {
    let client = unsafe { &mut *cptr };
    let mut client = client.lock().unwrap();
    
    if let Ok(s) = str_from_ptr(s) {
        client.connect(s);
    }
    client.stream.is_some() as u8
}
#[no_mangle]
pub extern fn client_disconnect(cptr: *mut Arc<Mutex<Client>>) -> u8 {
    let client = unsafe { &mut *cptr };
    let mut client = client.lock().unwrap();
    
    client.shutdown();
    client.stream.is_none() as u8
}


#[no_mangle]
pub extern fn client_login(cptr: *mut Arc<Mutex<Client>>) -> u8 {
    let client = unsafe { &*cptr };
    
    Client::login(client) as u8
}
#[no_mangle]
pub extern fn client_register(cptr: *mut Arc<Mutex<Client>>) {
    let client = unsafe { &mut *cptr };
    let mut client = client.lock().unwrap();
    
    client.register();
}


/// these are highly OS dependent
#[no_mangle]
pub extern fn client_save(cptr: *mut Arc<Mutex<Client>>) -> bool {
    let client = unsafe { &mut *cptr };
    let client = client.lock().unwrap();
    
    Client::save(&client, "client.key")
}
#[no_mangle]
pub extern fn client_load(cptr: *mut Arc<Mutex<Client>>) -> bool {
    let client = unsafe { &mut *cptr };
    let mut client = client.lock().unwrap();
    
    if let Some(c) = Client::load_file("client.key") {
        client.base = c.base.clone();
        true
    }
    else { false }
}


#[no_mangle]
pub extern fn client_chat(cptr: *mut Arc<Mutex<Client>>, s: *const c_char) {
    let client = unsafe { &mut *cptr };
    let mut client = client.lock().unwrap();
    
    if let Ok(s) = str_from_ptr(s) {
        client.chat(s);
    }
}
#[no_mangle]
pub extern fn client_nick(cptr: *mut Arc<Mutex<Client>>, s: *const c_char) {
    let client = unsafe { &mut *cptr };
    let mut client = client.lock().unwrap();
    
    if let Ok(s) = str_from_ptr(s) {
        client.nick(s);
    }
}




//NOTE: this is meant to be polled on frame-tick
#[no_mangle]
pub extern fn get_client_chat(cptr: *mut Arc<Mutex<Client>>, chat: &mut MChatFrame) -> u16 {
    let client = unsafe { &mut *cptr };
    let mut client = client.lock().unwrap();

    if client.msg.len() > 0 {
        let (uuid, msg) = client.msg.remove(0);
        
        chat.id = uuid.as_bytes().clone();

        let bytes = msg.as_bytes();
        for (i,b) in bytes.iter().enumerate() {
            chat.msg[i] = *b;
        }

        return bytes.len() as u16
    }

    0
}


#[no_mangle]
pub extern fn is_client_connected (cptr: *mut Arc<Mutex<Client>>) -> u8 {
    let client = unsafe { & *cptr };
    let client = client.lock().unwrap();
    client.stream.is_some() as u8
}

#[no_mangle]
pub extern fn get_client_ping (cptr: *mut Arc<Mutex<Client>>) -> f32 {
    let client = unsafe { & *cptr };
    let client = client.lock().unwrap();
    client.ping_delta
}
