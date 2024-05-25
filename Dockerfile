FROM rust:latest

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release --bin mk-realestate

COPY . .

ENTRYPOINT ["/usr/src/app/target/release/mk-realestate"]
