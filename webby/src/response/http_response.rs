use std::collections::HashMap;

use crate::utility::http_headers::CONTENT_LENGTH;

pub struct HttpResponse {
    status_code: u32,
    status_message: String,
    body: String,
    headers: HashMap<String, String>,
}

impl HttpResponse {
    /// Construct a new `HttpResponse`
    /// Defaults to a 200 OK response without headers and a body
    pub fn new() -> Self {
        Self {
            body: "".to_owned(),
            headers: HashMap::new(),
            status_code: 200,
            status_message: "Ok".to_owned(),
        }
    }

    pub fn continue_(mut self) -> Self {
        self.status_code = 100;
        self.status_message = "Continue".to_owned();
        self
    }

    pub fn switching_protocol(mut self) -> Self {
        self.status_code = 101;
        self.status_message = "Switching Protocol".to_owned();
        self
    }

    pub fn processing(mut self) -> Self {
        self.status_code = 102;
        self.status_message = "Processing".to_owned();
        self
    }

    pub fn early_hints(mut self) -> Self {
        self.status_code = 103;
        self.status_message = "Early Hints".to_owned().to_owned();
        self
    }

    pub fn ok(mut self) -> Self {
        self.status_code = 200;
        self.status_message = "Ok".to_owned();
        self
    }

    pub fn created(mut self) -> Self {
        self.status_code = 201;
        self.status_message = "Created".to_owned();
        self
    }

    pub fn accepted(mut self) -> Self {
        self.status_code = 202;
        self.status_message = "Accepted".to_owned();
        self
    }

    pub fn non_authoritative_information(mut self) -> Self {
        self.status_code = 203;
        self.status_message = "Non Authoritative Information".to_owned();
        self
    }

    pub fn no_content(mut self) -> Self {
        self.status_code = 204;
        self.status_message = "No Content".to_owned();
        self
    }

    pub fn reset_content(mut self) -> Self {
        self.status_code = 205;
        self.status_message = "Reset Content".to_owned();
        self
    }

    pub fn partial_content(mut self) -> Self {
        self.status_code = 206;
        self.status_message = "Partial Content".to_owned();
        self
    }

    pub fn multi_status(mut self) -> Self {
        self.status_code = 207;
        self.status_message = "Multi-Status".to_owned();
        self
    }

    pub fn already_reported(mut self) -> Self {
        self.status_code = 208;
        self.status_message = "Already Reported".to_owned();
        self
    }

    pub fn im_used(mut self) -> Self {
        self.status_code = 226;
        self.status_message = "IM Used".to_owned();
        self
    }

    pub fn multiple_choice(mut self) -> Self {
        self.status_code = 300;
        self.status_message = "Multiple Choice".to_owned();
        self
    }

    pub fn moved_permanently(mut self) -> Self {
        self.status_code = 301;
        self.status_message = "Moved Permanently".to_owned();
        self
    }

    pub fn found(mut self) -> Self {
        self.status_code = 302;
        self.status_message = "Found".to_owned();
        self
    }

    pub fn see_other(mut self) -> Self {
        self.status_code = 303;
        self.status_message = "See Other".to_owned();
        self
    }

    pub fn not_modified(mut self) -> Self {
        self.status_code = 304;
        self.status_message = "Not Modified".to_owned();
        self
    }

    pub fn use_proxy(mut self) -> Self {
        self.status_code = 305;
        self.status_message = "Use Proxy".to_owned();
        self
    }

    pub fn unused(mut self) -> Self {
        self.status_code = 306;
        self.status_message = "Unused".to_owned();
        self
    }

    pub fn temporary_redirect(mut self) -> Self {
        self.status_code = 307;
        self.status_message = "Temporary Redirect".to_owned();
        self
    }

    pub fn permament_redirect(mut self) -> Self {
        self.status_code = 308;
        self.status_message = "Permanent Redirect".to_owned();
        self
    }

    pub fn bad_request(mut self) -> Self {
        self.status_code = 400;
        self.status_message = "Bad Request".to_owned();
        self
    }

    pub fn unauthorized(mut self) -> Self {
        self.status_code = 401;
        self.status_message = "Unauthorized".to_owned();
        self
    }

    pub fn payment_required(mut self) -> Self {
        self.status_code = 402;
        self.status_message = "Payment Required".to_owned();
        self
    }

    pub fn forbidden(mut self) -> Self {
        self.status_code = 403;
        self.status_message = "Forbidden".to_owned();
        self
    }

    pub fn not_found(mut self) -> Self {
        self.status_code = 404;
        self.status_message = "Not Found".to_owned();
        self
    }

    pub fn method_not_allowed(mut self) -> Self {
        self.status_code = 405;
        self.status_message = "Method Not Allowed".to_owned();
        self
    }

    pub fn not_acceptable(mut self) -> Self {
        self.status_code = 406;
        self.status_message = "Not Acceptable".to_owned();
        self
    }

    pub fn proxy_authentication_required(mut self) -> Self {
        self.status_code = 407;
        self.status_message = "Proxy Authentication Required".to_owned();
        self
    }

    pub fn request_timeout(mut self) -> Self {
        self.status_code = 408;
        self.status_message = "Request Timeout".to_owned();
        self
    }

    pub fn conflict(mut self) -> Self {
        self.status_code = 409;
        self.status_message = "Conflict".to_owned();
        self
    }

    pub fn gone(mut self) -> Self {
        self.status_code = 410;
        self.status_message = "Gone".to_owned();
        self
    }

    pub fn length_required(mut self) -> Self {
        self.status_code = 411;
        self.status_message = "Length Required".to_owned();
        self
    }

    pub fn precondition_failed(mut self) -> Self {
        self.status_code = 412;
        self.status_message = "Precondition Failed".to_owned();
        self
    }

    pub fn payload_too_large(mut self) -> Self {
        self.status_code = 413;
        self.status_message = "Payload Too large".to_owned();
        self
    }

    pub fn uri_too_long(mut self) -> Self {
        self.status_code = 414;
        self.status_message = "URI Too Long".to_owned();
        self
    }

    pub fn unsupported_media_type(mut self) -> Self {
        self.status_code = 415;
        self.status_message = "Unsupported Media Type".to_owned();
        self
    }

    pub fn range_not_satisfiable(mut self) -> Self {
        self.status_code = 416;
        self.status_message = "Range Not Satisfiable".to_owned();
        self
    }

    pub fn expectation_failed(mut self) -> Self {
        self.status_code = 417;
        self.status_message = "Expectation Failed".to_owned();
        self
    }

    pub fn im_a_teapot(mut self) -> Self {
        self.status_code = 418;
        self.status_message = "I'm a teapot".to_owned();
        self
    }

    pub fn misdirected_request(mut self) -> Self {
        self.status_code = 421;
        self.status_message = "Misdirected Request".to_owned();
        self
    }

    pub fn unprocessable_entity(mut self) -> Self {
        self.status_code = 422;
        self.status_message = "Unprocessable Entity".to_owned();
        self
    }

    pub fn locked(mut self) -> Self {
        self.status_code = 423;
        self.status_message = "Locked".to_owned();
        self
    }

    pub fn failed_dependency(mut self) -> Self {
        self.status_code = 424;
        self.status_message = "Failed Dependency".to_owned();
        self
    }

    pub fn too_early(mut self) -> Self {
        self.status_code = 425;
        self.status_message = "Too Early".to_owned();
        self
    }

    pub fn upgrade_required(mut self) -> Self {
        self.status_code = 426;
        self.status_message = "Upgrade Required".to_owned();
        self
    }

    pub fn precondition_required(mut self) -> Self {
        self.status_code = 428;
        self.status_message = "Precondition Required".to_owned();
        self
    }

    pub fn too_many_requests(mut self) -> Self {
        self.status_code = 429;
        self.status_message = "Too Many Requests".to_owned();
        self
    }

    pub fn request_header_fields_too_large(mut self) -> Self {
        self.status_code = 431;
        self.status_message = "Request Header Fields Too Large".to_owned();
        self
    }

    pub fn unavailable_for_legal_reasons(mut self) -> Self {
        self.status_code = 451;
        self.status_message = "Unavailable For Legal Reasons".to_owned();
        self
    }

    pub fn internal_server_error(mut self) -> Self {
        self.status_code = 500;
        self.status_message = "Internal Server Error".to_owned();
        self
    }

    pub fn not_implemented(mut self) -> Self {
        self.status_code = 501;
        self.status_message = "Not Implemented".to_owned();
        self
    }

    pub fn bad_gateway(mut self) -> Self {
        self.status_code = 502;
        self.status_message = "Bad Gateway".to_owned();
        self
    }

    pub fn service_unavailable(mut self) -> Self {
        self.status_code = 503;
        self.status_message = "Service Unavailable".to_owned();
        self
    }

    pub fn gateway_timeout(mut self) -> Self {
        self.status_code = 504;
        self.status_message = "Gateway Timeout".to_owned();
        self
    }

    pub fn http_version_not_supported(mut self) -> Self {
        self.status_code = 505;
        self.status_message = "HTTP Version Not Supported".to_owned();
        self
    }

    pub fn variant_also_negotiates(mut self) -> Self {
        self.status_code = 506;
        self.status_message = "Variant Also Negotiates".to_owned();
        self
    }

    pub fn insufficient_storage(mut self) -> Self {
        self.status_code = 507;
        self.status_message = "Insufficient Storage".to_owned();
        self
    }

    pub fn loop_detected(mut self) -> Self {
        self.status_code = 508;
        self.status_message = "Loop Detected".to_owned();
        self
    }

    pub fn not_extended(mut self) -> Self {
        self.status_code = 510;
        self.status_message = "Not Extended".to_owned();
        self
    }

    pub fn network_autentication_required(mut self) -> Self {
        self.status_code = 511;
        self.status_message = "Network Autentication Required".to_owned();
        self
    }

    pub fn body(mut self, body: &str) -> Self {
        self.headers.insert(CONTENT_LENGTH.to_owned(), body.to_owned().as_bytes().len().to_string());
        self.body = body.to_owned();
        self
    }

    pub fn add_header(mut self, name: &str, value: &str) -> Self {
        self.headers.insert(name.to_owned(), value.to_owned());
        self
    }
}

impl std::fmt::Display for HttpResponse {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut headers = "".to_owned();

        for (index, header) in self.headers.iter().enumerate() {
            headers.push_str(header.0);
            headers.push_str(": ");
            headers.push_str(header.1);

            if index < self.headers.len() - 1 {
                headers.push_str("\r\n");
            }
        }

        let mut http_response = String::new();
        http_response.push_str(&format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_message));
        
        if !headers.is_empty() {
            http_response.push_str(&format!("{}\r\n", headers));
        }

        if !self.body.is_empty() {
            http_response.push_str(&format!("\r\n{}", self.body));
        }

        fmt.write_str(&http_response)
    }
}
