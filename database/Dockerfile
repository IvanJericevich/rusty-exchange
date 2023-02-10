# Rust official image (First stage)
FROM rustlang/rust:nightly AS builder
# Copy source files
COPY . .
# Run build --locked
RUN cargo build --release --bin database_manager --locked

# Debian lightweight image (Second stage)
FROM debian:buster-slim
# Install required SSL packages
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# Copy binary from first stage
COPY --from=builder /target/release/database_manager ./target/release/database_manager
# Execute binary
CMD ["/target/release/database_manager"]