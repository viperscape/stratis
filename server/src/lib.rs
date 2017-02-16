extern crate stratis_shared as shared;

mod server;
mod distributor;
mod store;

pub use server::Server;
pub use shared::client::Client;
