.PHONY: build download convert clean

build:
	cargo build --release --all-targets

download:
	rye run --pyproject download/pyproject.toml download

convert:
	cargo run --manifest-path convert/Cargo.toml --release

clean:
	cargo clean
