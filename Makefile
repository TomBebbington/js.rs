.PHONY: all build interactive tests doc clean
all: build interactive tests doc
build:
	cd src && rustc lib.rs --opt-level 3
	mv src/librust* bin
tests:
	cd src && rustc tests.rs -L ../bin --test --opt-level 0
	mv src/tests bin
interactive:
	cd src && rustc interactive.rs -L ../bin
	mv src/interactive bin/rust_js
doc:
	cd src && rustdoc lib.rs -o ../doc
clean:
	rm -rf bin/*