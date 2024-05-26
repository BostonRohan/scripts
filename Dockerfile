FROM rust:latest as builder

ARG MLS_LISTINGS_URL
ARG MK_REALESTATE_LISTINGS_API_TOKEN
ARG MK_REALESTATE_LISTINGS_API_URL
ARG RUST_LOG

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release --bin mk-realestate

FROM debian:buster-slim

ENV MLS_LISTINGS_URL=$MLS_LISTINGS_URL
ENV MK_REALESTATE_LISTINGS_API_TOKEN=$MK_REALESTATE_LISTINGS_API_TOKEN
ENV MK_REALESTATE_LISTINGS_API_URL=$MK_REALESTATE_LISTINGS_API_URL
ENV RUST_LOG=$RUST_LOG

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/mk-realestate /usr/local/bin/mk-realestate

CMD ["mk-realestate"]
