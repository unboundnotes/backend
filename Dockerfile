FROM rustlang/rust:nightly AS base
RUN cargo install cargo-watch

WORKDIR /app
# I actually want to mount the FS, not copy
COPY . .

EXPOSE 8000

CMD ["cargo", "watch", "-x", "run"]
