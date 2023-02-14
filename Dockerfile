# Using the nightly version of rust
FROM rustlang/rust:nightly

# set the working directory to /app inside the container
WORKDIR /app

# copy necessary files for building/running the application
COPY *.toml .env ./
COPY src ./src

# setup the app
RUN cargo check
RUN cargo build
RUN cargo install diesel_cli --no-default-features --features postgres
RUN diesel setup

EXPOSE 8080
CMD ["cargo", "run"]

