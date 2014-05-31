RUSTC ?= rustc
RUSTDOC ?= rustdoc
LIBDIR ?= /usr/lib/llvm-3.5/lib
.PHONY: all build doc genllvmdeps libs libllvm libjit libjs libjs_syntax update-doc clean
all: genllvmdeps libs build doc
genllvmdeps:
	mkdir -p target
	cd target && $(RUSTC) ../src/genllvmdeps/gen.rs -L .
	target/genllvmdeps src/libllvm/llvmdeps.rs
libjs:
	mkdir -p target
	cd target && $(RUSTC) ../src/libjs/lib.rs -L .
libjs_syntax:
	mkdir -p target
	cd target && $(RUSTC) ../src/libjs_syntax/lib.rs -L .
libllvm:
	mkdir -p target
	cd target && $(RUSTC) -O ../src/libllvm/lib.rs -L . -L $(LIBDIR)
libjit:
	mkdir -p target
	cd target && $(RUSTC) ../src/libjit/lib.rs -L .
libs: libllvm libjit libjs_syntax libjs
build:
	mkdir -p target
	cd target && $(RUSTC) ../src/front/front.rs -L .
install:
	sudo cp -f target/js.rs /usr/local/bin/
	sudo cp -f target/libjs*.so /usr/local/lib
	sudo cp -f target/libjit*.so /usr/local/lib
	-sudo ln -s /usr/local/bin/js.rs /usr/bin/js.rs
doc:
	$(RUSTDOC) src/libllvm/lib.rs -o doc -L target
	$(RUSTDOC) src/libjit/lib.rs -o doc -L target
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
