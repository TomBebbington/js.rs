RUSTC ?= rustc
RUSTDOC ?= rustdoc
.PHONY: all build doc libs libjit libjit_macro libjs libjs_syntax libjs_jit update-doc clean
all: libs build doc
libjs:
	mkdir -p target
	cd target && $(RUSTC) ../src/libjs/js.rs -L .
libjs_syntax:
	mkdir -p target
	cd target && $(RUSTC) ../src/libjs_syntax/js_syntax.rs -L .
libjs_jit:
	mkdir -p target
	cd target && $(RUSTC) ../src/libjs_jit/js_jit.rs -L .
libjit:
	mkdir -p target
	cd libs/jit && make build
	cp libs/jit/target/* target
libs: libjit libjs_syntax libjs libjs_jit
build:
	mkdir -p target
	cd target && $(RUSTC) ../src/js.rs/js.rs -L .
install:
	sudo cp -f target/js.rs /usr/local/bin/
	sudo cp -f target/libjs*.so /usr/local/lib
	sudo cp -f target/libjit*.so /usr/local/lib
	-sudo ln -s /usr/local/bin/js.rs /usr/bin/js.rs
doc:
	$(RUSTDOC) src/libjs/js.rs -o doc -L target
	$(RUSTDOC) src/libjs_syntax/js_syntax.rs -o doc -L target
	$(RUSTDOC) src/libjs_jit/js_jit.rs -o doc -L target
	$(RUSTDOC) src/js.rs/js.rs -o doc -L target
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
