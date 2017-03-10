use opcode::Opcode;
use uuid::Uuid;
use client::ID_LEN;

pub const BYTE_LEN: usize = ID_LEN + 1;

#[derive(Debug)]
pub struct Event (pub Opcode,
                  pub Uuid);

impl Event {
    /// convert event to an FFI friendly byte array
    pub fn to_bytes(self) -> [u8;BYTE_LEN] {
        let mut bytes = [0;BYTE_LEN];
        bytes[0] = self.0;
        for (i,b) in self.1.as_bytes().iter().enumerate() {
            bytes[i+1] = *b;
        }

        
        bytes
    }
}
