// @description String unit tests
hello = "Hello, world!";
assert(hello.length == 13, "String length");
assert(hello[2] == 'l', "String index");
assert(hello.charAt(1) == 'e', "String charAt");
assert(("Hello, "+'world!') == hello, "String concatenation and equality");
assert(String.fromCharCode(65,66,67) == "ABC", "String.fromCharCode");