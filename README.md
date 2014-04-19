[![Build Status](https://travis-ci.org/TopHattedCoder/rust_js.svg?branch=master)](https://travis-ci.org/TopHattedCoder/rust_js)
This is a generic JS parser and interpreter written in Rust. Currently, it has very limited support for the JS language.

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
+ URI decoding / encoding (partial, doesn't use JS rules)

Roadmap
-------
+ Add Date
+ Add SyntaxError, TypeError, etc

Contributing
------------
If you want to contribute just file a pull request with your changes. If it passes travis and is reasonably clean and justifiable, it will be merged. Look at the lints specified in src/lib.rs for coding guidelines.