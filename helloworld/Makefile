prog=$(shell basename $(CURDIR))

build:
	cargo build --verbose
	cp target/debug/$(prog) $(prog).day

test:
	cargo test --verbose

clean:
	cargo clean --verbose
	- rm $(prog).day

all: build

.PHONY: build test all clean