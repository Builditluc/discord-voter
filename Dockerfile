ARG BASE_IMAGE=rust:1.51.1-slim-buster

FROM $BASE_IMAGE as builder
WORKDIR /code

COPY Cargo.toml .
COPY Cargo.lock .
COPY src/ ./src

RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=builder /code/target/release/discord_voter /
COPY .env .env

CMD ["./discord_voter"]
