The plan for the LLVM JIT is that:

	+ All types are inferred
	+ If a value can be two types, bind the types!

For example:
```33 || 51.1;``` is binded as a double|int so it is stored in a double-sized box with a bit set if it is an int
```n > 0 ? 1 : null``` is binded as an int|null so it is a 32-bit int with a bit set if it is null

The following:
```
function fib(n) {
	return n <= 1 ? 1 : n * fib(n - 1);
}
```
Should compile to:
```
fn fib(n: int) -> int {
	return n <= 1 ? 1 : n * fib(n - 1);
}
```
After it infers that n is an int after seein that it is compared to multiplied by only ints