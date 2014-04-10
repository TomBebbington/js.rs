[![Build Status](https://travis-ci.org/TopHattedCoder/rust_js.svg?branch=master)](https://travis-ci.org/TopHattedCoder/rust_js)
This is a generic JS parser and interpreter written in Rust. Currently, it has very, very limited support for the JS language.

Documentation
-------------
[View here](http://www.rust-ci.org/TopHattedCoder/rust_js/doc/rust_js/)

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
+ Math
+ Number
+ JSON
+ Console.log

Roadmap
-------
+ Add Date

Contributing
------------
If you want to contribute just file a pull request with your changes. If it passes travis and is reasonably clean and justifiable, it will be merged. Look at the lints specified in src/lib.rs for coding guidelines.