# ---- Stage 1: Builder ----
FROM rust:1.85 as builder
WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    gcc \
    musl-tools \
    && rm -rf /var/lib/apt/lists/*

# Copy everything at once to avoid caching issues
COPY . .

# Build the application
RUN cargo build --release

# ---- Stage 2: Runtime ----
FROM debian:bookworm-slim
WORKDIR /app

# Install runtime dependencies needed for SQLx (TLS, date/time, etc.)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    tzdata \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary
COPY --from=builder /app/target/release/rust_api .
COPY --from=builder /app/migrations ./migrations

# Optionally copy migrations if you want to run inside container

EXPOSE 3000

ENV RUST_LOG=info
CMD ["./rust_api"]
