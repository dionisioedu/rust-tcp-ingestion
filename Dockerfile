FROM rust:slim as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/rust_tcp_ingestion .

ENV RUST_LOG=info

CMD ["./rust_tcp_ingestion"]