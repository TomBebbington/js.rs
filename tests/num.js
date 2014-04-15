// @description Number unit tests
assert(1 + 2 == 3);
assert(Number.isFinite(2));
assert(!Number.isFinite(NaN));
assert(!Number.isFinite(Infinity));
assert(5 / 2 == 2.5);
assert("2.5" == 2.5);
assert("2.5" !== 2.5);
assert(2 * 5 == 10);