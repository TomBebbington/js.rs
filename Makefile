.PHONY: all build doc tests interactive clean
all: build tests interactive doc
build:
	cargo-compile --manifest-path Cargo.toml
tests:
	rustc src/bin/tests.rs -L target -o target/tests --test
interactive:
	rustc src/bin/interactive.rs -L target -o target/interactive
doc:
	rustdoc src/script.rs -o doc
clean:
	rm -rf target/*