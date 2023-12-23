#
# Build stage
#
FROM rust:latest AS builder
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /build
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --locked --release

#
# Final stage
#
FROM scratch
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/typing-challenge /
COPY --from=builder /build/input.txt /
ENTRYPOINT ["/typing-challenge"]
