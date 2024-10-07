.PHONY: build run clean

build:
	cargo build --release --all-targets

run:
	rm -rf problems
	parallel --tty -j0 --halt-on-error now,fail=1 ::: geckodriver 'cargo run --release'

clean:
	cargo clean
