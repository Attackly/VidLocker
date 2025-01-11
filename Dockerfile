# Stage 1: Build the Rust application
FROM rust:1.84-alpine AS builder

# Install necessary build dependencies
RUN apk add --no-cache musl-dev libc-dev gcc openssl openssl-dev libssl3 libcrypto3
RUN apk add --no-cache libgcc libstdc++ bash openssl yt-dlp openssl-dev libssl3
RUN apk add --no-cache openssl-dev make
RUN apk add --no-cache \
    openssl-dev \
    musl-dev \
    gcc \
    libc-dev \
    make \
    bash \
    pkgconfig

# Set up the environment variable to specify the OpenSSL directory (if necessary)
ENV OPENSSL_LIB_DIR=/usr/lib
ENV OPENSSL_INCLUDE_DIR=/usr/include
# Set the working directory
WORKDIR /app

# Copy the source code
COPY . .
ENV SQLX_OFFLINE=true
# Build the application in release mode
RUN cargo install sqlx-cli
RUN cargo build --release

# Stage 2: Create a minimal runtime image
FROM alpine:latest

# Install runtime dependencies

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/VidLocker /app/vidlocker

# Expose necessary ports
EXPOSE 3000

# Run the application
CMD ["./vidlocker"]
