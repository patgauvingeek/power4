# Stage 1: Builder
FROM rust:alpine AS builder
WORKDIR /usr/app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --bin connect-four

# Stage 2: Final minimal image
FROM debian:buster-slim
WORKDIR /usr/local/bin
COPY --from=builder /usr/app/target/release/connect-four ./
CMD ["./connect-four"]
