# =====CONFIGURATION=====
# Version of postgresql to compile libpq from
PQ_VER  ?= 11.12
# OpenSSL version
SSL_VER ?= 1.1.1k
# Dumb-init version
DI_VER  ?= 1.2.5


# =====AUTO-GENERATED VARIABLES=====
# This is such a lovely oneliner
# NOTE: $(dir PATH) outputs a trailing slash
OUT_DIR     ?= $(dir $(abspath $(lastword $(MAKEFILE_LIST))))out

PREFIX      := $(OUT_DIR)/prefix
OPENSSL_DIR := $(OUT_DIR)/openssl-$(SSL_VER)
PQ_DIR      := $(OUT_DIR)/postgresql-$(PQ_VER)
DI_DIR      := $(OUT_DIR)/dumb-init-$(DI_VER)

# Used in various make calls to specify parallel recipes
CORES       != nproc


# =====ENVIRONMENT VARIABLES=====
export CC=musl-gcc -fPIC -pie -static
export LD_LIBRARY_PATH=$(PREFIX)
export PKG_CONFIG_PATH=/usr/local/lib/pkgconfig


# TODO check for header files (openssl-dev, libpq-dev) both for Arch & Ubuntu

# Create the out dir
$(shell mkdir -p "$(PREFIX)")


# ====RECIPES====
.PHONY: all
all: build

# libpq builds openssl as a dependency
.PHONY: build
build: libpq

.PHONY: clean
clean:
	@ echo "Note: this only cleans the C dependencies, not the Cargo cache."
	rm -rf "$(PQ_DIR)" "$(OPENSSL_DIR)" "$(DI_DIR)" "$(PREFIX)"


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
.PHONY: openssl
openssl: $(OPENSSL_DIR)/Configure
	C_INCLUDE_PATH="$(PREFIX)/include" $(MAKE) -C "$(OPENSSL_DIR)" depend
	$(MAKE) -C "$(OPENSSL_DIR)" -j$(CORES)
	$(MAKE) -C "$(OPENSSL_DIR)" install_sw


# =====LIBPQ=====
# Download the source code & configure the project
$(PQ_DIR)/configure:
	curl -sSL "https://ftp.postgresql.org/pub/source/v$(PQ_VER)/postgresql-$(PQ_VER).tar.gz" | \
		tar -C "$(OUT_DIR)" -xz
	cd "$(PQ_DIR)" && \
		LDFLAGS="-L$(PREFIX)/lib" CFLAGS="-I$(PREFIX)/include" ./configure \
			--without-readline \
			--with-openssl \
			--without-zlib \
			--prefix="$(PREFIX)" \
			--host=x86_64-unknown-linux-musl

.PHONY: libpq
libpq: openssl $(PQ_DIR)/configure
	make -C "$(PQ_DIR)/src/interfaces/libpq" -j$(CORES) all-static-lib
	make -C "$(PQ_DIR)/src/interfaces/libpq" install install-lib-static
	make -C "$(PQ_DIR)/src/bin/pg_config" -j $(CORES)
	make -C "$(PQ_DIR)/src/bin/pg_config" install


# =====DUMB-INIT=====
# NOTE: this is only used inside the Docker image, but it's here for completeness.
$(DI_DIR)/Makefile:
	curl -sSL "https://github.com/Yelp/dumb-init/archive/refs/tags/v$(DI_VER).tar.gz" | \
		tar -C "$(OUT_DIR)" -xz

.PHONY: dumb-init
dumb-init: $(DI_DIR)/Makefile
	make -C "$(DI_DIR)" build
