ARG RUST_VERSION=1.74.1

FROM rust:${RUST_VERSION}-slim-bullseye AS builder
WORKDIR /build
COPY . .
CMD ["cargo", "run"]
