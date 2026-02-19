FROM rust:1.91.1 AS builder

WORKDIR /usr/src/serverwiz

COPY . .

RUN cargo install --path .

FROM fedora:43 AS packwiz

RUN dnf install -y \
    ca-certificates \
    golang \
    && dnf clean all

RUN go install github.com/packwiz/packwiz@latest

FROM packwiz AS runner

COPY --from=builder /usr/local/cargo/bin/serverwiz /usr/local/bin/serverwiz

CMD ["serverwiz"]
