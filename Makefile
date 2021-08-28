PQ_VER ?= 11.12
SSL_VER ?= 1.1.1k

# This is such a lovely oneliner
# NOTE: $(dir PATH) outputs a trailing slash
OUT_DIR ?= $(dir $(abspath $(lastword $(MAKEFILE_LIST))))out/deps

# Generated variables for ease of use
PREFIX := $(OUT_DIR)/prefix
OPENSSL_DIR := $(OUT_DIR)/openssl-$(SSL_VER)
PQ_DIR := $(OUT_DIR)/postgresql-$(PQ_VER)
CORES != nproc

export CC=musl-gcc -fPIC -pie -static
export LD_LIBRARY_PATH=$(PREFIX)
export PKG_CONFIG_PATH=/usr/local/lib/pkgconfig


# TODO check for header files (openssl-dev, libpq-dev) both for Arch & Ubuntu

# Create the out dir
$(shell mkdir -p "$(PREFIX)")

all: openssl
.PHONY: all


# =====OPENSSL=====
# Download the source code & configure the project
$(OPENSSL_DIR)/Configure:
	curl -sSL "https://www.openssl.org/source/openssl-$(SSL_VER).tar.gz" | \
		tar -C "$(OUT_DIR)" -xz
	cd "$(OPENSSL_DIR)" && \
		CC="$$CC -idirafter /usr/include" ./Configure \
			no-zlib \
			no-shared \
			--prefix="$(PREFIX)" \
			--openssldir="$(PREFIX)/ssl" \
			linux-x86_64

# Build OpenSSL
openssl: $(OPENSSL_DIR)/Configure
	C_INCLUDE_PATH="$(PREFIX)/include" $(MAKE) -C "$(OPENSSL_DIR)" depend
	$(MAKE) -C "$(OPENSSL_DIR)" -j$(CORES)
	$(MAKE) -C "$(OPENSSL_DIR)" install_sw
.PHONY: openssl


# =====LIBPQ=====
# Download the source code & configure the project
$(PQ_DIR)/configure:
	curl -sSL "https://ftp.postgresql.org/pub/source/v$(PQ_VER)/postgresql-$(PQ_VER).tar.gz" | \
		tar -C "$(OUT_DIR)" -xz
	cd "$(PQ_DIR)" && \
		LDFLAGS="-L$(PREFIX)/lib" CFLAGS="-I$(PREFIX)/include" ./configure \
			--without-readline \
			--without-zlib \
			--with-openssl \
			--prefix="$(PREFIX)" \
			--host=x86_64-unknown-linux-musl

libpq: $(PQ_DIR)/configure
	make -C "$(PQ_DIR)/src/interfaces/libpq" -j$(CORES) all-static-lib
	make -C "$(PQ_DIR)/src/interfaces/libpq" install install-lib-static
	make -C "$(PQ_DIR)/src/bin/pg_config" -j $(CORES)
	make -C "$(PQ_DIR)/src/bin/pg_config" install
.PHONY: libpq
