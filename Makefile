.PHONY: fmt lint build test

fmt:
	cargo fmt

lint:
	cargo clippy

build:
	cargo build

test:
	cargo test
