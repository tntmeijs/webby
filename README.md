![version 0.0.5](https://img.shields.io/badge/version-0.0.5-red)

# Webby
Webby is a simple Rust web server that supports HTTP 1.1.
The main goal of this project is to provide a very simple web server.

## Motivation
The Rust ecosystem comes with a lot of third-party web server crates.
A lot of crates are absolutely brilliant, but none of them really teach you how a web server works.
This crate allows me to discover what makes a server tick, how to handle requests, asynchronous programming, and much, much more!

Right now, I wouldn't recommend using this crate in any serious projects. It's simply too unreliable and insecure.
But hey, maybe this crate will once day be listed on [AWWY](https://arewewebyet.org) if I manage to turn it into something usable... ;)

## Pull request guide
1. Create a branch from `master` using the following format: `feature/<your-feature-name-here>`.
2. Make your changes.
3. Write unit tests.
4. Test your changes.
5. Rebase on `master`.
6. Update the status badge in `README.md`.
7. Open a pull request.
8. Add a proper changelog / pull request description.
