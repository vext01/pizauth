PREFIX ?= /usr/local
MAN_PREFIX ?= ${PREFIX}/man

all:
	cargo build --release

install:
	cargo build --release
	install -d ${PREFIX}/bin
	install -c -m 555 target/release/pizauth ${PREFIX}/bin/pizauth
	install -d ${MAN_PREFIX}/man1
	install -d ${MAN_PREFIX}/man5
	install -c -m 444 pizauth.1 ${MAN_PREFIX}/man1/pizauth.1
	install -c -m 444 pizauth.conf.5 ${MAN_PREFIX}/man5/pizauth.conf.5
	install -d ${PREFIX}/share/examples/pizauth
	install -c -m 444 pizauth.conf.example ${PREFIX}/share/examples/pizauth

distrib:
	test "X`git status --porcelain`" = "X"
	@read v?'pizauth version: ' \
	  && mkdir pizauth-$$v \
	  && cp -rp Cargo.lock Cargo.toml COPYRIGHT LICENSE-APACHE LICENSE-MIT \
	    Makefile CHANGES.md README.md build.rs pizauth.1 pizauth.conf.5 \
	    pizauth.conf.example src pizauth-$$v \
	  && tar cfz pizauth-$$v.tgz pizauth-$$v \
	  && rm -rf pizauth-$$v
