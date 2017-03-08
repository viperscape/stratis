use std::sync::mpsc::Receiver;
use shared::events::{Event};


#[no_mangle]
pub extern fn poll_event (rx: *mut Receiver<Event>, event: &mut [u8]) -> u8 {
    let rx = unsafe { & *rx };
    if let Ok(e) = rx.try_recv() {
        for (i,b) in e.to_bytes().iter().enumerate() {
            event[i] = *b; //NOTE: expects event to be sized same
        }
        
        return true as u8
    }

    return false as u8
}

