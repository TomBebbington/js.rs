.PHONY: all build doc interactive tests clean
all: build interactive tests doc
build:
	cd target && rustc ../src/script.rs -L .
tests:
	rustc src/bin/tests.rs -L target -o target/tests
interactive:
	rustc src/bin/interactive.rs -L target -o target/interactive
doc:
	rustdoc src/script.rs -o doc
clean:
	rm -rf target/*