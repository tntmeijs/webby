use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};

pub struct Server {
    address: String
}

impl Server {
    pub fn new(address: &str) -> Server {
        Server {
            address: address.to_owned()
        }
    }

    pub fn start_listening(&self) {
        match TcpListener::bind(&self.address) {
            Ok(listener) => {
                for stream in listener.incoming() {
                    match stream {
                        Ok(mut stream) => self.handle_connection(&mut stream),
                        Err(error) => println!("ERROR: {}", error.to_string())
                    }
                }
            },
            Err(error) => println!("ERROR: {}", error.to_string())
        }
    }

    fn handle_connection(&self, stream: &mut TcpStream) {
        let mut buffer = [0; 1024];
        if stream.read(&mut buffer).is_err() {
            return println!("Unable to read buffer from incoming TCP stream");
        }

        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

        // TODO: move this into its own class
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        if stream.write(response.as_bytes()).is_err() {
            return println!("Unable to write response to TCP stream");
        }

        if stream.flush().is_err() {
            return println!("Unable to flush TCP stream");
        }
    }
}
