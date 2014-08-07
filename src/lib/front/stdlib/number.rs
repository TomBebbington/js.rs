use front::stdlib::value::{Value, ResultValue, VNumber, VInteger, to_value, from_value};
use front::stdlib::function::Function;
use std::f64::{NAN, MAX_VALUE, MIN_VALUE, INFINITY, NEG_INFINITY, EPSILON};
/// Parse a float into a value
pub fn parse_float(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    let parsed = from_str::<f64>(from_value::<String>(args[0]).unwrap().as_slice());
    return Ok(to_value(match parsed {
        Some(v) => v,
        None => NAN
    }));
}
/// Parse an int into a value
pub fn parse_int(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    let parsed = from_str::<i32>(from_value::<String>(args[0]).unwrap().as_slice());
    return Ok(match parsed {
        Some(v) => to_value(v),
        None => to_value(NAN)
    });
}
/// Check if a value when converted to a number is finite
pub fn is_finite(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() == 0 {
        false
    } else {
        from_value::<f64>(args[0]).unwrap().is_finite()
    }))
}
/// Check if a number is finite
pub fn strict_is_finite(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() == 0 {
        false
    } else {
        match *args[0] {
            VNumber(v) => v.is_finite(),
            VInteger(_) => true, // integers can't be infinite
            _ => false
        }
    }))
}
/// Check if a value when converted to a number is equal to NaN
pub fn is_nan(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() == 0 {
        false
    } else {
        from_value::<f64>(args[0]).unwrap().is_nan()
    }))
}
/// Check if a number is equal to NaN
pub fn strict_is_nan(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() == 0 {
        false
    } else {
        match *args[0] {
            VNumber(v) => v.is_nan(),
            _ => false
        }
    }))
}
/// Create a new `Number` object
pub fn _create(global:Value) -> Value {
    js!(global, {
        "NaN": NAN,
        "MAX_VALUE": MAX_VALUE,
        "MIN_VALUE": MIN_VALUE,
        "POSITIVE_INFINITY": INFINITY,
        "NEGATIVE_INFINITY": NEG_INFINITY,
        "EPSILON": EPSILON,
        "parseFloat": Function::make(parse_float, ["string"]),
        "parseInt": Function::make(parse_int, ["string"]),
        "isFinite": Function::make(strict_is_finite, ["num"]),
        "isNaN": Function::make(strict_is_nan, ["num"])
    })
}
/// Initialise the parse functions and `Number` on the global object
pub fn init(global:Value) {
    js_extend!(global, {
        "NaN": NAN,
        "Infinity": INFINITY,
        "parseFloat": Function::make(parse_float, ["string"]),
        "parseInt": Function::make(parse_int, ["string"]),
        "isFinite": Function::make(is_finite, ["number"]),
        "isNaN": Function::make(is_nan, ["num"]),
        "Number": _create(global)
    });
}