.PHONY: all build doc libs libjs libjs_syntax update-doc clean
all: libs build doc
libjs:
	mkdir -p target
	cd target && rustc ../src/libjs/lib.rs -L .
libjs_syntax:
	mkdir -p target
	cd target && rustc ../src/libjs_syntax/lib.rs -L .
libs: libjs_syntax libjs
build:
	mkdir -p target
	cd target && rustc ../src/front/front.rs -L .
doc:
	rustdoc src/libjs/lib.rs -o doc -L target
	rustdoc src/libjs_syntax/lib.rs -o doc -L target
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
