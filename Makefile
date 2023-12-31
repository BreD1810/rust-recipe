.PHONY: fmt lint build test doc

fmt:
	cargo fmt

lint:
	cargo clippy --all-features

build:
	cargo build --all-features

test:
	cargo test --all-features

doc:
	cargo doc --all-features --open
