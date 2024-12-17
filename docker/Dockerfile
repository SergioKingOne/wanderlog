FROM rust:1.82.0-slim as builder

# Install OpenSSL development packages and pkg-config
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty shell project
WORKDIR /usr/src/app
COPY . .

# Build the application with SQLx offline mode
ENV SQLX_OFFLINE=true
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install SSL libraries required at runtime
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/wanderlog .

# Copy environment file
COPY app/.env.example ./.env

# Expose port
EXPOSE 5000

# Run the binary
CMD ["./wanderlog"]