# Stolen and modified from https://gist.github.com/jlgerber/0f280236c2ee1b741dfe41a38d39a467
prog :=xnixperms

debug ?=

$(info debug is $(debug))

ifdef debug
  release :=
  target :=debug
  extension :=debug
else
  release :=--release
  target :=release
  extension :=
endif

build:
	cargo build $(release)

test:
	cargo test

all: build

help:
	@echo "usage: make $(prog) [debug=1]"