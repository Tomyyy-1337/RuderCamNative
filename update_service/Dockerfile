# syntax=docker/dockerfile:1.7

ARG BUILD_PLATFORM=linux/amd64
FROM --platform=${BUILD_PLATFORM} rust:latest AS builder

RUN dpkg --add-architecture arm64 && \
    apt-get update && apt-get install -y \
    gcc-aarch64-linux-gnu \
    libc6-dev-arm64-cross \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add aarch64-unknown-linux-gnu

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
ENV CARGO_PROFILE_RELEASE_STRIP=symbols

WORKDIR /app/update_service

COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/update_service/target \
    cargo build --release --target aarch64-unknown-linux-gnu && \
    cp /app/update_service/target/aarch64-unknown-linux-gnu/release/update_service /tmp/update_service

FROM alpine:latest
COPY --from=builder /tmp/update_service /update_service

# Build
# docker build -f Dockerfile -t pi-update-service . ; docker create --name temp pi-update-service ; docker cp temp:/update_service ./update_service_bin ; docker rm temp
