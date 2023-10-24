.PHONY: install

install:
	cargo build --release
	rm -r ~/.cargo/bin/streak
	cp target/release/streak ~/.cargo/bin
