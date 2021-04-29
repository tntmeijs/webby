use webby;

fn main() {
    webby::create("127.0.0.1", 8080)
        .start_listening();
}
