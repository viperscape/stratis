#[cfg(feature = "interface")]
extern crate imgui;
#[cfg(feature = "interface")]
extern crate glium;
#[cfg(feature = "interface")]
extern crate imgui_glium_renderer;

extern crate lichen;
extern crate uuid;

pub mod chat;
pub mod client;
pub mod player;
pub mod opcode;
pub mod events;

#[cfg(feature = "interface")]
pub mod interface;

pub use uuid::Uuid;
