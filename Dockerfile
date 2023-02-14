FROM rustlang/rust:nightly

WORKDIR /app
COPY *.toml .env ./
COPY src ./src

RUN ls -la

EXPOSE 8080
RUN cargo check
RUN cargo build

CMD ["cargo", "run"]

