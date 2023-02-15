# Using the nightly version of rust
FROM rustlang/rust:nightly
EXPOSE 8080

# set the working directory to /app inside the container
WORKDIR /app

# copy necessary files for building/running the application
COPY *.toml start-container.sh ./
COPY src ./src
COPY data ./data

RUN ls -larth

# setup the app
RUN cargo install --path .
RUN cargo build
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch
