# Build frontend files
FROM node:16 AS fbuilder

WORKDIR /usr/src/app

COPY web/ ./

RUN yarn install && \
    yarn build


# Build backend & backend docs
FROM rust:1.55-alpine AS builder

ARG DI_VER=1.2.5

# ENV OPENSSL_STATIC=1 \
#     PQ_LIB_STATIC=1

RUN apk update && \
    apk add --no-cache \
        postgresql \
        postgresql-dev \
        openssl-dev \
        build-base

WORKDIR /usr/src/app

# Build backend
COPY .cargo/ ./.cargo
COPY src/ ./src
COPY migrations/ ./migrations
COPY Cargo.toml Cargo.lock ./

RUN cargo build --release && \
    cargo doc --no-deps

# Build dumb-init
RUN curl -sSL "https://github.com/Yelp/dumb-init/archive/refs/tags/v$DI_VER.tar.gz" | \
    tar -xzf - && \
    cd "dumb-init-$DI_VER" && \
    make build && \
    mv dumb-init ..


FROM alpine:3.14.2

RUN mkdir -p /var/www/html

COPY --from=fbuilder /usr/src/app/dist /var/www/html/site
COPY --from=builder /usr/src/app/out/target/doc /var/www/html/doc
COPY --from=builder /usr/src/app/out/target/release/rbd /usr/bin/rbd
COPY --from=builder /usr/src/app/dumb-init /usr/bin/dumb-init

ENTRYPOINT [ "dumb-init", "--" ]
CMD [ "/usr/bin/rbd" ]

# RUN apt update && \
#     apt install -y --no-install-recommends \
#         musl-dev \
#         musl-tools \
#         libpq-dev \
#         libssl-dev && \
#     rustup target add x86_64-unknown-linux-musl && \
#     mkdir "$PREFIX" && \
#     echo "$PREFIX/lib" >> /etc/ld-musl-x86_64.path
