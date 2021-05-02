use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use log::{debug, error, info, trace};

use crate::{request::http_method::HttpMethod, response::http_response::HttpResponse};

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
        trace!("About to start listening on {}", self.address);
        match TcpListener::bind(&self.address) {
            Ok(listener) => {
                for stream in listener.incoming() {
                    match stream {
                        Ok(mut stream) => self.handle_connection(&mut stream),
                        Err(error) => error!("{}", error.to_string()),
                    }
                }
            }
            Err(error) => error!("{}", error.to_string()),
        }

        info!("Server running on \"{}\"", self.address);
        self
    }

    pub fn add_route(mut self, method: HttpMethod, pattern: &str, route_func: RouteFunc) -> Self {
        trace!("Adding pattern \"{}\"", pattern);

        if self.routing_patterns.contains_key(pattern) {
            debug!(
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
            return error!("Unable to read buffer from incoming TCP stream");
        }

        let route_to_execute = self.parse_request(&buffer);

        if route_to_execute.is_none() {
            trace!("Did not find a matching route to execute");
            self.write_response(
                stream,
                HttpResponse::new().not_found().to_string().as_bytes(),
            );
        } else {
            trace!("Executing matching route now");
            let response = route_to_execute.unwrap();
            self.write_response(stream, response.to_string().as_bytes());
        }
    }

    fn parse_request(&self, buffer: &[u8]) -> Option<HttpResponse> {
        let request_str = String::from_utf8_lossy(buffer).to_string();

        let mut parts = request_str.split_whitespace();
        let method = parts.next();
        let route = parts.next();
        let http_spec = parts.next();

        if method.is_none() || route.is_none() || http_spec.is_none() {
            error!("Malformed HTTP request");
            return None;
        }

        // Naive route matching algorithm
        for (pattern, route_info) in &self.routing_patterns {
            if route.unwrap() == pattern {
                if HttpMethod::from(method.unwrap()) == route_info.method {
                    debug!(
                        "Route \"{}\" matches pattern \"{}\"",
                        route.unwrap(),
                        pattern
                    );
                    return Some((route_info.callback)());
                } else {
                    debug!(
                        "Route \"{}\" matches pattern \"{}\", but the method \"{}\" is not allowed",
                        route.unwrap(),
                        pattern,
                        method.unwrap()
                    );
                    return Some(HttpResponse::new().method_not_allowed());
                }
            }
        }

        None
    }

    fn write_response(&self, stream: &mut TcpStream, data: &[u8]) {
        if stream.write(data).is_err() {
            return error!("Unable to write response to TCP stream");
        }

        if stream.flush().is_err() {
            return error!("Unable to flush TCP stream");
        }
    }
}
