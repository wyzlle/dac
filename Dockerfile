FROM rust:1.66.0-slim-buster as builder
WORKDIR /app

COPY . .
RUN cargo build --locked --release


FROM ubuntu as runner
WORKDIR /app

COPY --from=builder /app/target/release/dac /usr/local/bin/dac
COPY --from=builder /app/servers.json .

CMD ["dac"]