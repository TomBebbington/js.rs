.PHONY: all build interactive tests doc clean
all: build interactive tests doc
build:
	mkdir -p bin
	rustc src/lib.rs --out-dir=bin --opt-level=3
tests:
	rustc src/tests.rs --out-dir=bin -L bin --test --opt-level=3
interactive:
	rustc src/interactive.rs --out-dir=bin -L bin --opt-level=3
doc:
	rustdoc src/lib.rs -o doc
clean:
	rm -rf bin/*