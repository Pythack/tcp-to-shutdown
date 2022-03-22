FROM rust:latest

WORKDIR /tcpts

COPY ./Cargo.toml ./
COPY ./src/ ./src

RUN cargo build
RUN cp target/debug/tcpts /bin

WORKDIR /