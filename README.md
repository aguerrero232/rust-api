# Rust API 

REST API written in Rust

why?
-   to learn Rust
-   Rust is fast
-   no need for a virtual environment
-   did I mention it's fast?

In Rust, the dependency versions are specified per-project, and they are downloaded and compiled in the project build directory. There's nothing like pip install or sudo pip install because it's not needed, and every project is self-contained.

Even more, the specific dependency versions are saved to a lock file (Cargo.lock), so if you ask for foo = "1.0" and get foo = "1.2" and you add that file to VCS, your collaborators will also get foo 1.2 instead of whatever the latest version is when they get the code.

## Project Dependencies
*  [Rust](https://www.rust-lang.org/)
*  [PostgreSQL](https://www.postgresql.org/), sql database
*  [Diesel](http://diesel.rs/), ORM for Rust
*  [Actix](https://actix.rs/), web framework for Rust
* [cargo-watch](https://github.com/watchexec/cargo-watch), watches for changes in code during development



## Getting Started

* run `cargo build` to build the project
* run  `cargo watch -x run` to run the project and watch for changes
* run `cargo test` to run the tests
* run `cargo doc --open` to generate the documentation and open it in a browser