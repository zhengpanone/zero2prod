FROM rust:1.90-slim as builder

WORKDIR /app

# Install required dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy source code
COPY . .

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:stable-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -u 1001 appuser

# Copy binary from builder
COPY --from=builder /app/target/release/zero2prod /app/zero2prod
# Copy migrations
COPY --from=builder /app/migrations /app/migrations
# Copy .env
COPY .env .env

# Change ownership
RUN chown -R appuser:appuser /app

USER appuser


EXPOSE 8080
CMD ["./zero2prod"]
