[![Build Status](https://travis-ci.org/TomBebbington/js.rs.svg?branch=master)](https://travis-ci.org/TomBebbington/js.rs)
This is a Javascript lexer, parser and Just-in-Time compiler written in Rust. Currently, it has support for some of the language.

Documentation
-------------
[View here](http://tombebbington.github.io/js.rs/js/index.html)

Contributing
------------
If you want to contribute just file a pull request with your changes. If it passes travis and is reasonably clean and justifiable, it will be merged. Look at the lints specified in `src/libjs/lib.rs` for coding guidelines.

Building
--------
To build this project, clone it then run `make libs all` in the project root

Installing
----------
To install this project, run `[sudo ]make install` in the project root

Running
-------
+ To run the test suite, run `js.rs test`
+ To run the interactive (REPL) interpreter, run `js.rs interactive`
+ To run a specific script, run `js.rs *[script path]*`
