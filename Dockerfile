FROM rustlang/rust:nightly AS base
RUN cargo install cargo-watch

WORKDIR /app
# I actually want to mount the FS, not copy
COPY . .
CMD ["cargo", "watch", "-x", "run"]