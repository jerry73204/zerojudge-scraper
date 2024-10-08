.PHONY: build download convert clean

build:
	cargo build --release --all-targets

download:
	rm -rf problems
	parallel --tty -j0 --halt-on-error now,fail=1 ::: geckodriver 'cargo run --bin download --release'

convert:
	cargo run --bin convert --release

clean:
	cargo clean
