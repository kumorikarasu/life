# Dev Image

FROM rust:1-buster as dev
RUN cargo install cargo-watch

WORKDIR /usr/src/app

COPY src  src
COPY Cargo.toml Cargo.toml

CMD ["cargo", "watch", "-x", "run", "-w", "src"]


# Rust Build Stage
FROM rust:1-buster as build

WORKDIR /usr/src/app

COPY . .
RUN cargo build --release


# Final Stage
FROM debian:buster-slim as prod

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY --from=build /usr/src/app/target/release/api .

CMD ["./api"]

