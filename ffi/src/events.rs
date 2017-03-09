use std::sync::mpsc::Receiver;
use shared::events::{Event,BYTE_LEN};
use std::slice;

#[no_mangle]
pub extern fn poll_event (rx: *mut Receiver<Event>, bytes: *mut u8) -> u8 {
    let rx = unsafe { & *rx };
    if let Ok(e) = rx.try_recv() {
        unsafe {
            let mut bytes = slice::from_raw_parts_mut(bytes, BYTE_LEN);
            for (i,b) in e.to_bytes().iter().enumerate() {
                bytes[i] = *b;
            }
        }
        
        return true as u8
    }

    return false as u8
}

