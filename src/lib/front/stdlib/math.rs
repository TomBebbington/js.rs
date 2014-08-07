use front::stdlib::value::{Value, ResultValue, to_value, from_value};
use front::stdlib::function::Function;
use std::rand::random;
use std::f64;

/// Get the absolute value of a number
pub fn abs(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() >= 1 {
        from_value::<f64>(args[0]).unwrap().abs()
    } else {
        f64::NAN
    }))
}
/// Get the arccos of a number
pub fn acos(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() >= 1 {
        from_value::<f64>(args[0]).unwrap().acos()
    } else {
        f64::NAN
    }))
}
/// Get the arcsine of a number
pub fn asin(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() >= 1 {
        from_value::<f64>(args[0]).unwrap().asin()
    } else {
        f64::NAN
    }))
}
/// Get the arctangent of a number
pub fn atan(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() >= 1 {
        from_value::<f64>(args[0]).unwrap().atan()
    } else {
        f64::NAN
    }))
}
/// Get the arctangent of a numbers
pub fn atan2(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() >= 1 {
        from_value::<f64>(args[0]).unwrap().atan2(args[1].to_num())
    } else {
        f64::NAN
    }))
}
/// Get the cubic root of a number
pub fn cbrt(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() >= 1 {
        from_value::<f64>(args[0]).unwrap().cbrt()
    } else {
        f64::NAN
    }))
}
/// Get lowest integer above a number
pub fn ceil(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() >= 1 {
        from_value::<f64>(args[0]).unwrap().ceil()
    } else {
        f64::NAN
    }))
}
/// Get the cosine of a number
pub fn cos(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() >= 1 {
        from_value::<f64>(args[0]).unwrap().cos()
    } else {
        f64::NAN
    }))
}
/// Get the power to raise the natural logarithm to get the number
pub fn exp(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() >= 1 {
        from_value::<f64>(args[0]).unwrap().exp()
    } else {
        f64::NAN
    }))
}
/// Get the highest integer below a number
pub fn floor(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() >= 1 {
        from_value::<f64>(args[0]).unwrap().floor()
    } else {
        f64::NAN
    }))
}
/// Get the natural logarithm of a number
pub fn log(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() >= 1 {
        from_value::<f64>(args[0]).unwrap().log(f64::consts::E)
    } else {
        f64::NAN
    }))
}
/// Get the maximum of several numbers
pub fn max(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    let mut max = f64::NEG_INFINITY;
    for arg in args.iter() {
        let num = arg.to_num();
        max = max.max(num);
    }
    Ok(to_value(max))
}
/// Get the minimum of several numbers
pub fn min(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    let mut max = f64::INFINITY;
    for arg in args.iter() {
        let num = arg.to_num();
        max = max.min(num);
    }
    Ok(to_value(max))
}
/// Raise a number to a power
pub fn pow(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() >= 2 {
        let num : f64 = from_value(args[0]).unwrap();
        let power : f64 = from_value(args[1]).unwrap();
        num.powf(power)
    } else {
        f64::NAN
    }))
}
/// Generate a random floating-point number between 0 and 1
pub fn _random(_:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(random::<f64>()))
}
/// Round a number to the nearest integer
pub fn round(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() >= 1 {
        from_value::<f64>(args[0]).unwrap().round()
    } else {
        f64::NAN
    }))
}
/// Get the sine of a number
pub fn sin(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() >= 1 {
        from_value::<f64>(args[0]).unwrap().sin()
    } else {
        f64::NAN
    }))
}
/// Get the square root of a number
pub fn sqrt(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() >= 1 {
        from_value::<f64>(args[0]).unwrap().sqrt()
    } else {
        f64::NAN
    }))
}
/// Get the tangent of a number
pub fn tan(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() >= 1 {
        from_value::<f64>(args[0]).unwrap().tan()
    } else {
        f64::NAN
    }))
}
/// Create a new `Math` object
pub fn _create(global : Value) -> Value {
    js!(global, {
        "E": f64::consts::E,
        "LN2": f64::consts::LN_2,
        "LN10": f64::consts::LN_10,
        "LOG2E": f64::consts::LOG2_E,
        "LOG10E": f64::consts::LOG10_E,
        "SQRT1_2": 0.5f64.sqrt(),
        "SQRT2": f64::consts::SQRT2,
        "PI": f64::consts::PI,
        "abs": Function::make(abs, ["num1", "num2"]),
        "acos": Function::make(acos, ["num1", "num2"]),
        "asin": Function::make(asin, ["num1", "num2"]),
        "atan": Function::make(atan, ["num1", "num2"]),
        "atan2": Function::make(atan2, ["num1", "num2"]),
        "cbrt": Function::make(cbrt, ["num1", "num2"]),
        "ceil": Function::make(ceil, ["num1", "num2"]),
        "cos": Function::make(cos, ["num1", "num2"]),
        "exp": Function::make(exp, ["num1", "num2"]),
        "floor": Function::make(floor, ["num"]),
        "log": Function::make(log, ["num1", "num2"]),
        "max": Function::make(max, ["num1", "num2"]),
        "min": Function::make(min, ["num1", "num2"]),
        "pow": Function::make(pow, ["num1", "num2"]),
        "random": Function::make(_random, []),
        "round": Function::make(round, ["num"]),
        "sin": Function::make(sin, ["num"]),
        "sqrt": Function::make(sqrt, ["num"]),
        "tan": Function::make(tan, ["num"])
    })
}
/// Initialise the `Math` object on the global object
pub fn init(global:Value) {
    js_extend!(global, {
        "Math": _create(global)
    });
}