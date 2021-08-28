PQ_VER ?= 11.12
SSL_VER ?= 1.1.1k

OUT_DIR ?= out/deps

export CC = musl-gcc -fPIC -pie -static

# TODO check for header files (openssl-dev, libpq-dev) both for Arch & Ubuntu


all: openssl
.PHONY: all


# =====OPENSSL=====
# Download the source code
$(OUT_DIR)/openssl-$(SSL_VER)/Configure:
	mkdir -p '$(OUT_DIR)'
	curl -sSL "https://www.openssl.org/source/openssl-$(SSL_VER).tar.gz" | \
		tar -C "$(OUT_DIR)" -xz

# Build OpenSSL
openssl: $(OUT_DIR)/openssl-$(SSL_VER)/Configure
.PHONY: openssl
