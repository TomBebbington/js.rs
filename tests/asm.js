// @description ASM tests
function DiagModule(stdlib, foreign, heap) {
    "use asm";

    // Variable Declarations
    var sqrt = stdlib.Math.sqrt;

    // Function Declarations
    function square(x) {
        x = +x;
        return +(x*x);
    }

    function diag(x, y) {
        x = +x;
        y = +y;
        return +sqrt(square(x) + square(y));
    }

    return { diag: diag };
}
diag = DiagModule({ Math: Math }).diag;
console.log(diag(10, 100))