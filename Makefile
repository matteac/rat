build:
	cargo build --release

install:
	cp -rf target/release/rat /usr/bin/rat

uninstall:
	rm -rf /usr/bin/rat

all: build install


check:
	rat --version