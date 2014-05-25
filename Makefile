.PHONY: all build doc update-doc interactive tests clean
all: build interactive tests doc
build:
	mkdir -p target
	cd target && rustc ../src/lib.rs -L .
tests:
	rustc src/bin/tests.rs -L target -o target/tests
interactive:
	rustc src/bin/interactive.rs -L target -o target/interactive
doc:
	rustdoc src/lib.rs -o doc
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
