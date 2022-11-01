build:
	cargo build --release

install:
	mkdir -p $(DESTDIR)$(PREFIX)/bin
	cp -f target/release/rat $(DESTDIR)/$(PREFIX)/bin/rat
	chmod 755 $(DESTDIR)$(PREFIX)/bin/rat


uninstall:
	rm -rf /usr/bin/rat

all: build install


check:
	rat --version