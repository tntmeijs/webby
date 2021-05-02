#[derive(PartialEq, Eq)]
pub enum HttpMethod {
    INVALID,
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl From<&str> for HttpMethod {
    fn from(str: &str) -> Self {
        match str.trim() {
            "GET" => Self::GET,
            "HEAD" => Self::HEAD,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "DELETE" => Self::DELETE,
            "CONNECT" => Self::CONNECT,
            "OPTIONS" => Self::OPTIONS,
            "TRACE" => Self::TRACE,
            "PATCH" => Self::PATCH,
            _ => Self::INVALID,
        }
    }
}
