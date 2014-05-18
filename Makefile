.PHONY: all build doc interactive tests clean
all: build interactive tests doc
build:
	rustc src/script.rs -L target -o target/libscript
tests:
	rustc src/bin/tests.rs -L target -o target/tests
interactive:
	rustc src/bin/interactive.rs -L target -o target/interactive
doc:
	rustdoc src/script.rs -o doc
clean:
	rm -rf target/*