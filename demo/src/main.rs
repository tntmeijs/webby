use webby;

fn main() {
    let server = webby::create("127.0.0.1", 8080);
    server.start_listening();
}
