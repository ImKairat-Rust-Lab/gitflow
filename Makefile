.PHONY: run build release test clippy format

run:
	@cargo run -q

build:
	@cargo build

release:
	@cargo build --release

test:
	@cargo test

test_release:
	@cargo test --release

format:
	@cargo fmt

clippy:
	@cargo clippy --fix

check-release:
	@cargo test --release
	@cargo fmt
	@cargo clippy
