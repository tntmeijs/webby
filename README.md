![version 0.0.5](https://img.shields.io/badge/version-0.0.5-red)

# Webby
Webby is a simple Rust web server that supports HTTP 1.1.

## Motivation
The Rust ecosystem comes with a lot of third-party web server crates.
A lot of crates are absolutely brilliant, but none of them really teach you how a web server works.
This crate allows me to discover how HTTP 1.1 works, how to handle requests, asynchronous programming, and much, much more!

Right now, I wouldn't recommend using this crate in any serious projects. It's simply too slow and unreliable.

## Features
### Create a simple HTTP server
```rust
// Run a server on localhost:8080
fn main() {
    webby::create("127.0.0.1", 8080)
        .start_listening();
}
```

### Routing
No web server would be complete without any sort of routing, which is why Webby supports routing too.

You can either use functions, or lambda functions. As long as you match the expected function signature, it'll work!

```rust
fn index() -> HttpResponse {
    println!("This is an index route, it only ever returns HTTP 204.");
    HttpResponse::new().no_content()
}

fn main() {
    webby::create("127.0.0.1", 8080)
        .add_route(HttpMethod::GET, "/", index)
        .start_listening();
}
```

## Pull request guide
1. Create a branch from `master` using the following format: `feature/<your-feature-name-here>`.
2. Make your changes.
3. Write unit tests.
4. Test your changes.
5. Rebase on `master`.
6. Update the status badge in `README.md`.
7. Open a pull request.
8. Add a proper changelog / pull request description.

## Third-party dependencies
I've tried to keep Webby as simple as possible. However, some tasks are just out of scope for this project.

### Demo
- [Env Logger](https://crates.io/crates/env_logger) - a logger that can be configured via environment variables
- [DotEnv](https://crates.io/crates/dotenv) - load environment variables from a `.env` file

### Webby
- [Log](https://crates.io/crates/log) - lightweight logging facade
