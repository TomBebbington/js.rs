RUSTC ?= rustc
RUSTDOC ?= rustdoc
.PHONY: all build doc libs libjs libjs_syntax update-doc clean
all: libs build doc
libjs:
	mkdir -p target
	cd target && $(RUSTC) ../src/libjs/lib.rs -L .
libjs_syntax:
	mkdir -p target
	cd target && $(RUSTC) ../src/libjs_syntax/lib.rs -L .
libjit:
	mkdir -p target
	cd target && $(RUSTC) ../src/libjit/lib.rs -L .
libs: libjit libjs_syntax libjs
build:
	mkdir -p target
	cd target && $(RUSTC) ../src/front/front.rs -L .
install:
	sudo cp -f target/js.rs /usr/local/bin/
	sudo cp -f target/libjs*.so /usr/local/lib
	-sudo ln -s /usr/local/bin/js.rs /usr/bin/js.rs
doc:
	$(RUSTDOC) src/libjs/lib.rs -o doc -L target
	$(RUSTDOC) src/libjs_syntax/lib.rs -o doc -L target
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
