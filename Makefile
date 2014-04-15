.PHONY: all build doc clean
all: build doc
build:
	cargo-compile --manifest-path Cargo.toml
doc:
	rustdoc src/script.rs -o doc
clean:
	rm -rf target/*