use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::{
    request::http_method::HttpMethod,
    response::http_response::HttpResponse,
    utility::{http_headers, mime_types},
};

pub type RouteFunc = fn() -> HttpResponse;

struct RouteInfo {
    method: HttpMethod,
    callback: RouteFunc,
}

pub struct Server {
    address: String,
    routing_patterns: HashMap<String, RouteInfo>,
}

impl Server {
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_owned(),
            routing_patterns: HashMap::new(),
        }
    }

    pub fn start_listening(self) -> Self {
        match TcpListener::bind(&self.address) {
            Ok(listener) => {
                for stream in listener.incoming() {
                    match stream {
                        Ok(mut stream) => self.handle_connection(&mut stream),
                        Err(error) => println!("ERROR: {}", error.to_string()),
                    }
                }
            }
            Err(error) => println!("ERROR: {}", error.to_string()),
        }

        self
    }

    pub fn add_route(mut self, method: HttpMethod, pattern: &str, route_func: RouteFunc) -> Self {
        println!("Adding pattern \"{}\"", pattern);

        if self.routing_patterns.contains_key(pattern) {
            println!(
                "Pattern \"{}\" already exists, routing information will be overwritten",
                pattern
            );
        }

        self.routing_patterns.insert(
            pattern.to_owned(),
            RouteInfo {
                method: method,
                callback: route_func,
            },
        );

        self
    }

    fn handle_connection(&self, stream: &mut TcpStream) {
        let mut buffer = [0; 1024];
        if stream.read(&mut buffer).is_err() {
            return println!("Unable to read buffer from incoming TCP stream");
        }

        self.parse_request(&buffer);

        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

        // For the time being, always return HTTP status 200
        let response = HttpResponse::new()
            .ok()
            .add_header(http_headers::CONTENT_TYPE, mime_types::TXT)
            .body("Hello, client! <3");

        if stream.write(response.to_string().as_bytes()).is_err() {
            return println!("Unable to write response to TCP stream");
        }

        if stream.flush().is_err() {
            return println!("Unable to flush TCP stream");
        }
    }

    fn parse_request(&self, buffer: &[u8]) {
        let request_str = String::from_utf8_lossy(buffer).to_string();

        let mut parts = request_str.split_whitespace();
        let method = parts.next();
        let route = parts.next();
        let http_spec = parts.next();

        if method.is_none() || route.is_none() || http_spec.is_none() {
            println!("ERROR: Malformed HTTP request");
            return;
        }

        println!("HTTP method: \"{}\", route: \"{}\", spec: \"{}\"", method.unwrap(), route.unwrap(), http_spec.unwrap());
    }
}
