FROM rust:1.50 as builder

RUN apt-get update -yqq && apt-get install -yqq cmake g++ libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres


RUN cargo new --bin web-rest
WORKDIR ./web-todo

COPY ./css ./css
COPY ./javascript ./javascript
COPY ./migrations ./migrations
COPY ./src ./src
COPY ./templates ./templates
COPY ./.env ./.env
COPY ./Cargo.toml ./Cargo.toml
COPY ./diesel.toml ./diesel.toml

RUN cargo build --release --bin web-rest

FROM debian:buster-slim

COPY ./templates /templates
COPY ./javascript /javascript
COPY ./css /css
COPY .env .env

COPY --from=builder /web-todo/target/release/web-rest /web-rest

RUN apt-get update -yqq && apt-get install -yqq libpq-dev

USER root

ENV PORT=8000
ENV RUST_LOG="info,parser::expression=info,actix_web=info"
ENV RUST_BACKTRACE=full

CMD ["/web-rest"]