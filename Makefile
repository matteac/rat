build:
	cargo build --release

install:
	cp -f target/release/rat $(DESTDIR)/bin/rat

uninstall:
	rm -rf /usr/bin/rat

all: build install


check:
	rat --version