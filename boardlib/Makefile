prog=$(shell basename $(CURDIR))

build:
	cargo build --verbose

test:
	cargo test --verbose

clean:
	cargo clean --verbose
	- rm $(prog).day

format:
	cargo fmt

lint:
	cargo fmt -- --check

all: build

.PHONY: build test all clean format lint
