FROM rust:1.91.1 AS builder

WORKDIR /usr/src/serverwiz

COPY . .

RUN cargo install --path .

FROM debian:stable-slim AS packwiz

RUN apt-get update && \
    apt-get install -y golang ca-certificates && \
    update-ca-certificates

RUN go install github.com/packwiz/packwiz@latest

FROM packwiz AS runner

COPY --from=builder /usr/local/cargo/bin/serverwiz /usr/local/bin/serverwiz

CMD ["serverwiz"]
