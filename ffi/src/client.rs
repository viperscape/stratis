use std::sync::{Arc, Mutex};
use std::slice;

use shared::client::Client;

use ::{MClientBase,MChatFrame,MPlayer,MClient,
       KEY_LEN,
       c_char,str_from_ptr};


#[no_mangle]
pub extern fn default_client(mc: &mut MClient) {
    let (client, rx) = Client::default();
    let client = Box::new(Arc::new(Mutex::new(client)));
    let rx = Box::new(rx);

    mc.client = Box::into_raw(client);
    mc.rx = Box::into_raw(rx);
}


#[no_mangle]
pub extern fn drop_mclient(mc: &mut MClient) {
    if !mc.client.is_null() { unsafe { Box::from_raw(mc.client); } }
    if !mc.rx.is_null() { unsafe { Box::from_raw(mc.rx); } }
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
    
    if let Some((c,_)) = Client::load_file("client.key") {
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


/// get player cache count, then supply sized array for get_players
#[no_mangle]
pub extern fn get_player_count (cptr: *mut Arc<Mutex<Client>>) -> u32 {
    let client = unsafe { & *cptr };
    let client = client.lock().unwrap();
    client.cache.len() as u32
}
#[no_mangle]
pub extern fn get_players (cptr: *mut Arc<Mutex<Client>>,
                           players: *mut MPlayer,
                           len: u32) {
    let client = unsafe { & *cptr };
    let client = client.lock().unwrap();
    let mut players = unsafe {
        slice::from_raw_parts_mut(players, len as usize)
    };

    for (i,p) in client.cache.iter().enumerate() {
        let nick = p.1.nick.as_bytes();

        for (k,c) in players[i].nick.iter_mut().enumerate() {
            *c = nick[k];
        }
    }
}
