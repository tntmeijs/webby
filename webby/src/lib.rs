use server::server::Server;

pub mod server;
pub mod response;
pub mod utility;

pub fn create(address: &str, port: u32) -> Server {
    Server::new(&format!("{}:{}", address, port))
}
