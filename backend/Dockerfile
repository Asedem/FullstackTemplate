FROM rust:1-slim AS builder

WORKDIR /usr/src/app

RUN apt-get update && \
    apt-get install -y --no-install-recommends libpq-dev pkg-config && \
    rm -rf /var/lib/apt/lists/*

COPY server ./server

WORKDIR /usr/src/app/server

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install -y --no-install-recommends libpq5 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/server/target/release/server /usr/local/bin/server

EXPOSE 8080

CMD ["/usr/local/bin/server"]
