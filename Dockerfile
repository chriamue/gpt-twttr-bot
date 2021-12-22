FROM rust:bullseye AS builder
WORKDIR /usr/src/

RUN USER=root cargo new gpt-twttr-bot
WORKDIR /usr/src/gpt-twttr-bot
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs
COPY src ./src
RUN touch src/main.rs
RUN cargo build --release
ENTRYPOINT [ "cargo", "run", "--release" ]
