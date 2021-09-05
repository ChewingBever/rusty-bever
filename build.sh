#!/usr/bin/env bash

set -e

# Install build dependencies
# apt update
# apt install \
#     -y --no-install-recommends \
#     musl-dev \
#     musl-tools \
#     libssl-dev \
#     libpq-dev

make
