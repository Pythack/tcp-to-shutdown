FROM rust:latest

WORKDIR /tcpts

COPY ./Cargo.toml ./
COPY ./src/ ./src

RUN cargo build
RUN cp target/debug/tcpts /bin

# Building and installing the binary
RUN \
    cargo build --release &&\
    cp ./target/release/tcpts /bin

# Making the tar.gz file
RUN \
    mkdir tcpts-x86_64-unknown-linux-gnu &&\
    mv ./target/release/tcpts tcpts-x86_64-unknown-linux-gnu &&\
    tar -czvf tcpts-x86_64-unknown-linux-gnu.tar.gz tcpts-x86_64-unknown-linux-gnu &&\
    rm -rf tcpts-x86_64-unknown-linux-gnu

WORKDIR /