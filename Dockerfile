# Stage 1: Builder
FROM rust:alpine AS builder
WORKDIR /usr/app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --bin power-4

# Stage 2: Final minimal image
FROM alpine
WORKDIR /usr/local/bin
COPY --from=builder /usr/app/target/release/power-4 ./
CMD ["./power-4"]
