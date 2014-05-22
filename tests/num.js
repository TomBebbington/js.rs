// @description Number unit tests
assert(3.0 == 3, "Number equality");
assert("3" == 3, "Number and string equality");
assert("4" !== 3, "Number and string strict equality");
assert(Number.isFinite(2), "Number.isFinite");
assert((5 + 2) == 7, "Number addition");
assert((5 - 2) == 3, "Number subtraction");
assert((5 / 2) == 2.5, "Number division");
assert((5 * 2) == 10, "Number multiplication");
