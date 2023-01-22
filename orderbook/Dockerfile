# Rust official image (First stage)
FROM rust:latest AS builder
# Copy source files
COPY . .
# Run build
RUN cargo build --release --bin matching_engine

# Debian lightweight image (Second stage)
FROM debian:buster-slim
# Copy binary from first stage
COPY --from=builder ./target/release/matching_engine ./target/release/matching_engine
# Execute binary
CMD ["/target/release/matching_engine"]