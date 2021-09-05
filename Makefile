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
export CC := musl-gcc -fPIC -pie -static
export LD_LIBRARY_PATH := $(PREFIX)
export PKG_CONFIG_PATH := /usr/local/lib/pkgconfig
export PATH := /usr/local/bin:/root/.cargo/bin:$(PATH)


# TODO check for header files (openssl-dev, libpq-dev) both for Arch & Ubuntu

# Create the out dir
$(shell mkdir -p "$(PREFIX)")


# =====BUILDING THE STATIC BINARY=====
.PHONY: all
all: build

.PHONY: builder
builder:
	docker build \
		-t rusty-builder:latest - < docker/Dockerfile.builder

.PHONY: docker
docker: builder
	docker run \
		--rm \
		-v "$$PWD:/usr/src" \
		--workdir "/usr/src" \
		-it \
		rusty-builder:latest \
		bash build.sh


# libpq builds openssl as a dependency
.PHONY: build
build: libpq

.PHONY: clean
clean: clean-openssl clean-libpq clean-di
	@ echo "Note: this only cleans the C dependencies, not the Cargo cache."
	rm -rf "$(PREFIX)"

# This is used inside the Dockerfile
.PHONY: pathfile
pathfile:
	echo "$(PREFIX)/lib" >> /etc/ld-musl-x86_64.path


## =====OPENSSL=====
# Download the source code & configure the project
$(OPENSSL_DIR)/Configure:
	curl -sSL "https://www.openssl.org/source/openssl-$(SSL_VER).tar.gz" | \
		tar -xzC "$(OUT_DIR)"
	cd "$(OPENSSL_DIR)" && \
		CC="$(CC) -idirafter /usr/include -idirafter /usr/include/x86_64-linux-gnu/" ./Configure \
			no-zlib \
			no-shared \
			--prefix="$(PREFIX)" \
			--openssldir="$(PREFIX)/ssl" \
			linux-x86_64

# Build OpenSSL
.PHONY: openssl
openssl: $(OPENSSL_DIR)/Configure
	cd "$(OPENSSL_DIR)" && env C_INCLUDE_PATH="$(PREFIX)/include" $(MAKE) depend 2> /dev/null
	cd "$(OPENSSL_DIR)" && $(MAKE) -j$(CORES)
	cd "$(OPENSSL_DIR)" && $(MAKE) install_sw

.PHONY: clean-openssl
clean-openssl:
	rm -rf "$(OPENSSL_DIR)"


## =====LIBPQ=====
# Download the source code & configure the project
$(PQ_DIR)/configure:
	curl -sSL "https://ftp.postgresql.org/pub/source/v$(PQ_VER)/postgresql-$(PQ_VER).tar.gz" | \
		tar -xzC "$(OUT_DIR)"
	cd "$(PQ_DIR)" && \
		LDFLAGS="-L$(PREFIX)/lib" CFLAGS="-I$(PREFIX)/include" ./configure \
			--without-readline \
			--with-openssl \
			--without-zlib \
			--prefix="$(PREFIX)" \
			--host=x86_64-unknown-linux-musl

.PHONY: libpq
libpq: openssl $(PQ_DIR)/configure
	cd "$(PQ_DIR)/src/interfaces/libpq" && $(MAKE) -j$(CORES) all-static-lib
	cd "$(PQ_DIR)/src/interfaces/libpq" && $(MAKE) install install-lib-static
	cd "$(PQ_DIR)/src/bin/pg_config" && $(MAKE) -j$(CORES)
	cd "$(PQ_DIR)/src/bin/pg_config" && $(MAKE) install

.PHONY: clean-libpq
clean-libpq:
	rm -rf "$(PQ_DIR)"


# =====DUMB-INIT=====
# NOTE: this is only used inside the Docker image, but it's here for completeness.
$(DI_DIR)/Makefile:
	curl -sSL "https://github.com/Yelp/dumb-init/archive/refs/tags/v$(DI_VER).tar.gz" | \
		tar -C "$(OUT_DIR)" -xz

.PHONY: di
di: $(DI_DIR)/Makefile
	make -C "$(DI_DIR)" build

.PHONY: clean-di
clean-di:
	rm -rf "$(DI_DIR)"


# ====UTILITIES FOR DEVELOPMENT=====
## The tests require a database, so we run them like this
test:
	docker-compose -f docker-compose.test.yml -p rb_test up
