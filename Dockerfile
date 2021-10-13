FROM rustlang/rust:nightly AS builder
WORKDIR /usr/src/

RUN USER=root cargo new gpt-twttr-bot
WORKDIR /usr/src/gpt-twttr-bot
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release
RUN cargo install --path .

FROM rustlang/rust:nightly

COPY --from=builder /usr/src/gpt-twttr-bot/target/release/gpt-twttr-bot .
RUN touch tweets.db
USER 1000
CMD [ "./gpt-twttr-bot" ]
