FROM rust:latest

WORKDIR /usr/src/bin/mk-realestate

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release --bin mk-realestate

COPY . .

ENTRYPOINT ["/usr/src/bin/target/release/mk-realestate"]
