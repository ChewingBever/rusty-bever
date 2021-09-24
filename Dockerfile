FROM rust:1.54

ENV PREFIX="/usr/src/out/prefix" \
    CC="musl-gcc -fPIC -pie -static" \
    LD_LIBRARY_PATH="$PREFIX" \
    PKG_CONFIG_PATH="/usr/local/lib/pkgconfig" \
    PATH="/usr/local/bin:/root/.cargo/bin:$PATH"

RUN apt update && \
    apt install -y --no-install-recommends \
        musl-dev \
        musl-tools \
        libpq-dev \
        libssl-dev && \
    rustup target add x86_64-unknown-linux-musl && \
    mkdir "$PREFIX" && \
    echo "$PREFIX/lib" >> /etc/ld-musl-x86_64.path

WORKDIR /usr/src/app