# vim: ft=dockerfile
FROM rust:1.54

ENV PREFIX="/usr/src/out/prefix" \
    CC="musl-gcc -fPIC -pie -static" \
    LD_LIBRARY_PATH="$PREFIX" \
    PKG_CONFIG_PATH="/usr/local/lib/pkgconfig" \
    PATH="/usr/local/bin:/root/.cargo/bin:$PATH"

WORKDIR /usr/src/app

RUN groupadd -g 1000 builder && \
    useradd -u 1000 -g 1000 builder && \
    mkdir -p "$PREFIX" && \
    chown -R builder:builder /usr/src/app && \
    apt update && \
    apt install -y --no-install-recommends \
        musl-dev \
        musl-tools \
        libpq-dev \
        libssl-dev && \
    rustup target add x86_64-unknown-linux-musl && \
    echo "$PREFIX/lib" >> /etc/ld-musl-x86_64.path


USER builder

CMD ["cargo", "test"]
