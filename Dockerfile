FROM rust:latest

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release --bin mk-realestate

ENTRYPOINT ["/usr/src/app/target/release/mk-realestate"]
