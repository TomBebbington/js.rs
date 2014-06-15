RUSTC ?= rustc
RUSTDOC ?= rustdoc
.PHONY: all build doc libs libs/jit src/back src/front src/main src/syntax update-doc clean
all: libs src build doc
src/back: src/front src/syntax libs/jit
	mkdir -p target
	cd target && $(RUSTC) ../$@/back.rs -L .
src/front: src/syntax
	mkdir -p target
	cd target && $(RUSTC) ../$@/front.rs -L .
src/main: src/back src/front libs/jit
	mkdir -p target
	cd target && $(RUSTC) ../$@/main.rs -L .
src/syntax:
	mkdir -p target
	cd target && $(RUSTC) ../$@/syntax.rs -L .
src: src/back src/front src/main src/syntax
build: src/main
libs/jit:
	mkdir -p target
	cd libs/jit && make build
	cp libs/jit/target/* target
libs: libs/jit
doc:
	$(RUSTDOC) src/syntax/syntax.rs -o doc -L target
	$(RUSTDOC) src/front/front.rs -o doc -L target
	$(RUSTDOC) src/back/back.rs -o doc -L target
	$(RUSTDOC) src/main/main.rs -o doc -L target
update-doc: doc
	rm -rf /tmp/doc
	mv doc /tmp/doc
	git checkout gh-pages
	rm -rf ./*
	mv /tmp/doc/* .
	-git add -A .
	-git commit -a -m "Auto-update docs"
	-git push origin gh-pages
	git checkout master
clean:
	rm -rf target/*
