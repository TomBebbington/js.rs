RUSTC ?= rustc
CARGO ?= cargo
RUSTDOC ?= rustdoc
.PHONY: all build doc update-doc clean
all: build doc
build:
	$(CARGO) build
clean:
	rm -rf target/*js*
