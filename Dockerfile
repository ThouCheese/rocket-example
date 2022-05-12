FROM rust:1.60.0
RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features postgres
RUN mkdir /workdir
WORKDIR /workdir
