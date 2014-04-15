//@description String unit tests
hello = "Hello, world!";
console.log(hello.length);
assert(hello[2] == 'l');
assert(hello.charAt(1) == 'e');
assert("Hello, "+'world!' == hello);
assert("String.fromCharCode(65,66,67)" == "ABC");
assert(hello.length == 13);