RUSTC ?= rustc
CARGO ?= cargo
RUSTDOC ?= rustdoc
.PHONY: all build doc update-doc clean
all: build doc
build:
	$(CARGO) build
doc:
	$(RUSTDOC) src/lib/lib.rs -o doc -L target/deps
update-doc: doc
	git checkout gh-pages
	rm -rf ./*
	mv /tmp/doc/* .
	-git add -A .
	-git commit -a -m "Auto-update docs"
	-git push origin gh-pages
	git checkout master
clean:
	rm -rf target/*js*
