use webby::request::http_method::HttpMethod;

fn test() {
    println!("GET called for route \"/test\", function as parameter works!");
}

fn main() {
    webby::create("127.0.0.1", 8080)
        .add_route(HttpMethod::GET, "/", || println!("GET called for route \"/\", lambda works!"))
        .add_route(HttpMethod::GET, "/test", test)
        .start_listening();
}
