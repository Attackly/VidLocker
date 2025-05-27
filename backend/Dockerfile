# Stage 1: Build the Rust application
FROM rust:1.87.0-alpine AS builder

# Install necessary build dependencies
RUN apk add --no-cache \
    musl-dev \
    libc-dev \
    gcc \
    libgcc \
    libstdc++ \
    openssl \
    openssl-dev \
    libssl3 \
    libcrypto3 \
    make \
    bash \
    pkgconfig

ENV OPENSSL_LIB_DIR=/usr/lib
ENV OPENSSL_INCLUDE_DIR=/usr/include
# Set the working directory
WORKDIR /app

# Copy the source code
COPY ./backend .
ENV SQLX_OFFLINE=true
# Build the application in release mode
RUN cargo build --release

# Stage 2: Create a minimal runtime image
FROM alpine:latest

RUN apk add yt-dlp

# Set the working directory
WORKDIR /app
# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/VidLocker /app/vidlocker

# Expose necessary ports
EXPOSE 3001

# Run the application
CMD ["./vidlocker"]
