FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

ARG MLS_LISTINGS_URL
ARG MK_REALESTATE_LISTINGS_API_TOKEN
ARG MK_REALESTATE_LISTINGS_API_URL

ENV MLS_LISTINGS_URL=$MLS_LISTINGS_URL
ENV MK_REALESTATE_LISTINGS_API_TOKEN=$MK_REALESTATE_LISTINGS_API_TOKEN
ENV MK_REALESTATE_LISTINGS_API_URL=$MK_REALESTATE_LISTINGS_API_URL

RUN echo "listings:" $MLS_LISTINGS_URL
RUN echo $MK_REALESTATE_LISTINGS_API_TOKEN
RUN echo $MK_REALESTATE_LISTINGS_API_URL

COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin mk-realestate

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/mk-realestate /usr/local/bin/mk-realestate

CMD ["/usr/local/bin/mk-realestate"]
