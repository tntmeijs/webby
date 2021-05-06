use webby::{request::{http_method::HttpMethod, request_data::RequestData}, response::http_response::HttpResponse};

use log::info;

fn test(_data: &RequestData) -> HttpResponse {
    info!("GET called for route \"/test\", function as parameter works!");
    HttpResponse::new().ok()
}

fn main() {
    dotenv::from_filename(format!("{}/.env", module_path!())).ok();
    env_logger::init();

    webby::create("127.0.0.1", 8080)
        .add_route(HttpMethod::GET, "/", | _data | {
            info!("GET called for route \"/\", lambda works!");
            HttpResponse::new().ok()
        })
        .add_route(HttpMethod::GET, "/test", test)
        .start_listening();
}
