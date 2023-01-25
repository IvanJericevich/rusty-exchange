# Rust official image (First stage)
FROM rustlang/rust:nightly AS builder
# Copy source files
COPY . .
# Run build --locked
RUN cargo build --release --bin api --locked

# Debian lightweight image (Second stage)
FROM debian:buster-slim
# Install required SSL packages
RUN apt update && apt-get install -y libssl-dev pkg-config openssl
# Copy binary from first stage
COPY --from=builder /target/release/api ./target/release/api
# Expose port
EXPOSE 8080
# Execute binary
CMD ["/target/release/api"]