FROM rustlang/rust:nightly AS base
RUN cargo install cargo-watch

WORKDIR /
RUN cargo new --bin app

WORKDIR /app
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./appconfig_derive ./appconfig_derive
RUN cargo build
RUN rm src/*.rs

COPY . .

EXPOSE 8000

CMD ["cargo", "watch", "-x", "run"]
