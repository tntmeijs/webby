use server::server::Server;

pub mod server;

pub fn create(address: &str, port: u32) -> Server {
    Server::new(&format!("{}:{}", address, port))
}
