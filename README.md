[![Build Status](https://travis-ci.org/TopHattedCoder/rust_js.svg?branch=master)](https://travis-ci.org/TopHattedCoder/rust_js)
This is a Javascript lexer, parser and interpreter written in Rust. Currently, it has support for some of the language.

Documentation
-------------
[View here](http://www.rust-ci.org/TopHattedCoder/rust_js/doc/script/)

Supported language features
---------------------------
+ Functions(partial, return last expression)
+ Switch blocks (partial, ignore breaks)
+ If blocks
+ While blocks
+ Object declarations
+ Array declarations
+ Strings
+ Numbers

Supported APIs
--------------
+ Error (partial)
+ Math
+ Number
+ JSON
+ Console
+ URI decoding / encoding (doesn't use JS rules)

Contributing
------------
If you want to contribute just file a pull request with your changes. If it passes travis and is reasonably clean and justifiable, it will be merged. Look at the lints specified in src/lib.rs for coding guidelines.

Building
--------
To build this project, clone it then run `make all` in the root

Running
-------
+ To run the test suite, run `target/tests` from the project root.
+ To run the interactive (REPL) interpreter, run `target/interactive` from the project root