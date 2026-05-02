.PHONY: run build release

run:
	@cargo run -q

build:
	@cargo build 

release:
	@cargo build --release
