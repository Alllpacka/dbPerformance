FROM rust:latest

WORKDIR /db_Performance

COPY Cargo.toml ./Cargo.toml

COPY src ./src

RUN cargo build

CMD ["cargo", "run"]
