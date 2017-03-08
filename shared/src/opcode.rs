pub type Opcode = u8;

pub const LOGIN: u8 = 0;
pub const REGISTER: u8 = 1;
pub const CHAT: u8 = 2;
pub const PLAYER: u8 = 3; //signifies update or add
pub const PLAYER_DROP: u8 = 4;

pub const PING: u8 = 10;
pub const PONG: u8 = 11;
