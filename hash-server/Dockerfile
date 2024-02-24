FROM rust:1.74-slim-bullseye as builder

WORKDIR /usr/src/hash-server
COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim

COPY --from=builder /usr/local/cargo/bin/hash-server /usr/local/bin/hash-server
RUN chmod +x /usr/local/bin/hash-server

CMD ["hash-server"]