# ⚙️ ***Rust API*** - `Getting Started`

## ***Prerequisites***

*Project Dependencies*
*  [Rust](https://www.rust-lang.org/), main programming language for the application
*  [PostgreSQL](https://www.postgresql.org/), sql database
*  [Diesel](http://diesel.rs/), ORM for Rust
*  [Actix](https://actix.rs/), web framework for Rust
* [cargo-watch](https://github.com/watchexec/cargo-watch), watches for changes in code during development

*Development Dependencies*
  *  [Docker](https://www.docker.com/), containerization platform
  *  [Docker Compose](https://docs.docker.com/compose/), tool for defining and running multi-container Docker applications

## ***Startup***

* clone the repository
    ```sh
    git clone https://github.com/aguerrero232/rust-api.git rust-api
    ```

  * change into the projects directory

      ```sh
      cd rust-api
      ```

* install rustup and toolchain

    ```shell
    rustup toolchain install stable
    ```

* compile the current package and all of its dependencies

    ```shell
    cargo build
    ```

* build the projects app container

    ```sh
    docker-compose build
    ```

* run the projects containers

    ```sh
    docker-compose up
    ```

* you can now access the app at `localhost:8080` with a postgres database running at `localhost:5432`