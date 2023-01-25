# Rust official image (First stage)
FROM rustlang/rust:nightly AS builder
# Copy source files
COPY . .
# Run build
RUN cargo build --release --bin matching_engine --locked

# Debian lightweight image (Second stage)
FROM debian:buster-slim
# Install required SSL packages
RUN apt update && apt-get install -y libssl-dev pkg-config openssl
# Copy binary from first stage
COPY --from=builder /target/release/matching_engine ./target/release/matching_engine
# Execute binary
CMD ["/target/release/matching_engine"]