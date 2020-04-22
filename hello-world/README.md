# Hello World example

This service is a minimal example of an API that simply returns the string `Hello, SomeName` in response to an HTTP request to `/hello/SomeName`.

It is intentionally simple in order to aid learning, where everything that is not essential for a running service is left out. There are tests (because these are seen as essential 😇).

Please feel free to fork this repo and use this example as you wish (including submitting PRs).

## To get going

The API is built with [Rust](https://www.rust-lang.org/), using a web server crate called [tide](https://github.com/http-rs/tide), and the source code can be seen in [./src/main.rs](./src/main.rs).

Rust is used because it helps us write safe, reliable and fast services. It also compiles to a single binary and is small and lightweight - ideal for containers. This example uses Tide because it's a simple and idiomatic web server that is familiar and easy to understand and use.

1. Install the rust toolchain (see https://www.rust-lang.org/learn/get-started)
1. Run the tests to make sure everything is set up correctly: `cargo test`
1. Run locally: `cargo run`
1. Build a Docker image `make docker-build`
1. Run docker image `make docker-run`
1. Fork the repo, and submit a PR if you want to
