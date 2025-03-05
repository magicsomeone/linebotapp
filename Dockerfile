FROM rust:latest AS builder

COPY . .

RUN cargo build --release

FROM rust:latest

WORKDIR /linebotapp

COPY --from=builder target/release/linebotapp-cli .
COPY --from=builder config ./config

CMD ["./linebotapp-cli", "start"]