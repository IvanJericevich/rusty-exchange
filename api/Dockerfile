# Rust official image (First stage)
FROM rust:latest AS builder
# Copy source files
COPY . .
# Run build
RUN cargo build --release

# Debian lightweight image (Second stage)
FROM debian:buster-slim
# Copy binary from first stage
COPY --from=builder ./target/release/api ./target/release/api
# Execute binary
CMD ["/target/release/api"]