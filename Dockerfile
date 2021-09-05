FROM rust:1.54

RUN apt update && \
    apt install -y --no-install-recommends \
        musl-dev \
        musl-tools \
        libpq-dev \
        libssl-dev && \
    rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app
