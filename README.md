[![Build Status](https://travis-ci.org/TopHattedCoder/rust_js.svg?branch=master)](https://travis-ci.org/TopHattedCoder/rust_js)
This is a generic JS parser and interpreter written in Rust. Currently, it has very, very limited support for the JS language.

Supported features
------------------
+ Math functions (fully supports the standard functions)
+ Switch blocks (partially implemented, they currently ignore break statements)
+ If blocks
+ While blocks
+ Object declarations
+ Array declarations
+ Strings
+ Numbers

Roadmap
-------
+ Add functions

Contributing
------------
If you want to contribute just file a pull request with your changes. If it passes travis and is reasonably clean and justifiable, it will be merged. Look at the lints specified in src/lib.rs for coding guidelines.